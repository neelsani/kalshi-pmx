//! RSA request signing helpers for Kalshi API credentials.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rsa::RsaPrivateKey;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::pss::BlindedSigningKey;
use rsa::rand_core::OsRng;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use sha2::Sha256;

use crate::error::{Error, Result};

/// Header carrying the Kalshi API key id.
pub const ACCESS_KEY_HEADER: &str = "KALSHI-ACCESS-KEY";
/// Header carrying the request signature.
pub const ACCESS_SIGNATURE_HEADER: &str = "KALSHI-ACCESS-SIGNATURE";
/// Header carrying the millisecond timestamp used in the signature payload.
pub const ACCESS_TIMESTAMP_HEADER: &str = "KALSHI-ACCESS-TIMESTAMP";

/// RSA signer for Kalshi API-key authentication.
///
/// The signer accepts PKCS#8 or PKCS#1 PEM keys. Its `Debug` implementation redacts the private
/// key material.
#[derive(Clone)]
pub struct ApiKeySigner {
    key_id: Arc<str>,
    private_key: Arc<RsaPrivateKey>,
}

impl std::fmt::Debug for ApiKeySigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiKeySigner")
            .field("key_id", &self.key_id)
            .field("private_key", &"<redacted>")
            .finish()
    }
}

impl ApiKeySigner {
    /// Creates a signer from an API key id and RSA private key PEM text.
    ///
    /// PEM text may contain literal `\n` escapes, which makes it convenient to load from `.env`.
    pub fn from_pem(key_id: impl Into<String>, pem: &str) -> Result<Self> {
        let pem = normalize_pem(pem);
        let key = RsaPrivateKey::from_pkcs8_pem(&pem)
            .or_else(|_| RsaPrivateKey::from_pkcs1_pem(&pem))
            .map_err(|err| Error::Auth(format!("failed to parse RSA private key: {err}")))?;

        Ok(Self {
            key_id: Arc::from(key_id.into()),
            private_key: Arc::new(key),
        })
    }

    /// Returns the configured API key id.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Signs a request method and path, returning the three Kalshi authentication headers.
    ///
    /// `path_without_query` should be the API path used for signing, without query parameters.
    pub fn sign_headers(&self, method: &str, path_without_query: &str) -> Result<AuthHeaders> {
        let timestamp = current_millis()?;
        let payload = format!(
            "{timestamp}{}{path_without_query}",
            method.to_ascii_uppercase()
        );
        let signature = self.sign(payload.as_bytes());

        Ok(AuthHeaders {
            key_id: self.key_id.to_string(),
            signature,
            timestamp,
        })
    }

    fn sign(&self, payload: &[u8]) -> String {
        let signing_key = BlindedSigningKey::<Sha256>::new((*self.private_key).clone());
        let signature = signing_key.sign_with_rng(&mut OsRng, payload);
        STANDARD.encode(signature.to_bytes())
    }
}

/// Kalshi authentication header values for one signed request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthHeaders {
    /// API key id header value.
    pub key_id: String,
    /// Base64-encoded RSA-PSS signature.
    pub signature: String,
    /// Millisecond timestamp header value.
    pub timestamp: String,
}

fn current_millis() -> Result<String> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| Error::Auth(format!("system clock is before UNIX epoch: {err}")))?
        .as_millis();
    Ok(millis.to_string())
}

fn normalize_pem(input: &str) -> String {
    let input = input.trim().replace("\\n", "\n");
    if input.contains('\n') {
        return input;
    }

    let Some(begin_prefix_start) = input.find("-----BEGIN ") else {
        return input;
    };
    let label_start = begin_prefix_start + "-----BEGIN ".len();
    let Some(label_end_rel) = input[label_start..].find("-----") else {
        return input;
    };

    let label_end = label_start + label_end_rel;
    let label = &input[label_start..label_end];
    let begin_marker = format!("-----BEGIN {label}-----");
    let end_marker = format!("-----END {label}-----");
    let body_start = begin_prefix_start + begin_marker.len();
    let Some(end_start) = input.find(&end_marker) else {
        return input;
    };

    let body = input[body_start..end_start]
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();

    let mut normalized = String::with_capacity(input.len() + body.len() / 64 + 4);
    normalized.push_str(&begin_marker);
    normalized.push('\n');
    for chunk in body.as_bytes().chunks(64) {
        normalized.push_str(std::str::from_utf8(chunk).expect("PEM base64 is ASCII"));
        normalized.push('\n');
    }
    normalized.push_str(&end_marker);
    normalized.push('\n');
    normalized
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use rsa::RsaPrivateKey;
    use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
    use rsa::rand_core::OsRng;

    use super::{ACCESS_KEY_HEADER, ApiKeySigner, normalize_pem};

    fn test_private_key_pem() -> String {
        RsaPrivateKey::new(&mut OsRng, 1024)
            .unwrap()
            .to_pkcs1_pem(LineEnding::LF)
            .unwrap()
            .to_string()
    }

    #[test]
    fn normalizes_single_line_pem() {
        let input = "-----BEGIN RSA PRIVATE KEY----- AAAA BBBB CCCC -----END RSA PRIVATE KEY-----";
        let output = normalize_pem(input);

        assert!(output.starts_with("-----BEGIN RSA PRIVATE KEY-----\n"));
        assert!(output.contains("\nAAAABBBBCCCC\n"));
        assert!(output.ends_with("-----END RSA PRIVATE KEY-----\n"));
    }

    #[test]
    fn signer_accepts_escaped_pem_and_redacts_debug() {
        let pem = test_private_key_pem().replace('\n', "\\n");
        let signer = ApiKeySigner::from_pem("key-id", &pem).unwrap();

        assert_eq!(signer.key_id(), "key-id");
        let debug = format!("{signer:?}");
        assert!(debug.contains("key-id"));
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("BEGIN RSA PRIVATE KEY"));
    }

    #[test]
    fn sign_headers_include_key_timestamp_and_base64_signature() {
        let signer = ApiKeySigner::from_pem("key-id", &test_private_key_pem()).unwrap();
        let headers = signer
            .sign_headers("get", "/trade-api/v2/exchange/status")
            .unwrap();

        assert_eq!(ACCESS_KEY_HEADER, "KALSHI-ACCESS-KEY");
        assert_eq!(headers.key_id, "key-id");
        assert!(headers.timestamp.parse::<u128>().is_ok());
        assert!(!STANDARD.decode(headers.signature).unwrap().is_empty());
    }
}
