use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_yaml::{Mapping, Value};

const HTTP_METHODS: &[&str] = &["get", "post", "put", "delete", "patch"];
const REGEN_COMMAND: &str = "cargo run --manifest-path xtask/Cargo.toml -- generate-openapi-types";

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

const STRING_LIKE_TYPES: &[&str] = &["String", "FixedPointCount", "FixedPointDollars"];

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-openapi-types") => generate_openapi_types(),
        Some(command) => Err(format!("unknown xtask command `{command}`").into()),
        None => Err(
            "usage: cargo run --manifest-path xtask/Cargo.toml -- generate-openapi-types".into(),
        ),
    }
}

fn generate_openapi_types() -> Result<()> {
    let root = repo_root();
    let spec_path = root.join("specs/openapi.yaml");
    let spec: Value = serde_yaml::from_str(&fs::read_to_string(&spec_path)?)?;
    let generator = Generator { spec: &spec };

    let generated_rs = root.join("src/generated.rs");
    let params_rs = root.join("src/params.rs");
    let typed_rs = root.join("src/typed.rs");

    fs::write(&generated_rs, generator.generated_rs()?)?;
    fs::write(&params_rs, generator.params_rs()?)?;
    fs::write(&typed_rs, generator.typed_rs()?)?;
    rustfmt(&[generated_rs, params_rs, typed_rs])?;

    Ok(())
}

fn rustfmt(paths: &[PathBuf]) -> Result<()> {
    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .args(paths)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("rustfmt failed with status {status}").into())
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives under repo root")
        .to_path_buf()
}

struct Generator<'a> {
    spec: &'a Value,
}

impl Generator<'_> {
    fn generated_rs(&self) -> Result<String> {
        let mut out = Vec::new();
        out.push("//! Generated OpenAPI component schemas.".to_owned());
        out.push(format!("//! Regenerate with `{REGEN_COMMAND}`."));
        out.push("#![allow(clippy::large_enum_variant)]".to_owned());
        out.push("#![allow(clippy::struct_field_names)]".to_owned());
        out.push("#![allow(clippy::module_name_repetitions)]".to_owned());
        out.push("#![allow(clippy::new_without_default)]".to_owned());
        out.push("#![allow(clippy::redundant_field_names)]".to_owned());
        out.push("#![allow(clippy::too_many_arguments)]".to_owned());
        out.push(String::new());

        for (name, schema) in self.schemas()? {
            out.push(self.emit_schema(name, schema));
            out.push(String::new());
        }

        Ok(out.join("\n"))
    }

    fn params_rs(&self) -> Result<String> {
        let mut out = Vec::new();
        out.push("//! Generated OpenAPI query parameter structs.".to_owned());
        out.push(format!("//! Regenerate with `{REGEN_COMMAND}`."));
        out.push(String::new());
        out.push("use crate::generated::*;".to_owned());
        out.push(String::new());

        let mut seen = BTreeSet::new();
        for (_path, path_item) in self.paths()? {
            let Some(path_item) = path_item.as_mapping() else {
                continue;
            };
            for method in HTTP_METHODS {
                let Some(operation) = mapping_get(path_item, method).and_then(Value::as_mapping)
                else {
                    continue;
                };
                if mapping_get(operation, "operationId").is_none() {
                    continue;
                }
                let type_name = self.operation_params_type(operation)?;
                if seen.contains(&type_name) {
                    continue;
                }
                let Some(emitted) = self.emit_params_struct(operation)? else {
                    continue;
                };
                seen.insert(type_name);
                out.push(emitted);
                out.push(String::new());
            }
        }

        Ok(out.join("\n"))
    }

    fn typed_rs(&self) -> Result<String> {
        let mut out = Vec::new();
        out.push("//! Generated typed Trade API methods.".to_owned());
        out.push(format!("//! Regenerate with `{REGEN_COMMAND}`."));
        out.push(String::new());
        out.push("use crate::generated::*;".to_owned());
        out.push(String::new());
        out.push("#[derive(Clone, Debug)]".to_owned());
        out.push("pub struct TypedClient {".to_owned());
        out.push("    client: crate::Kalshi,".to_owned());
        out.push("}".to_owned());
        out.push(String::new());
        out.push("impl TypedClient {".to_owned());
        out.push("    pub(crate) fn new(client: crate::Kalshi) -> Self {".to_owned());
        out.push("        Self { client }".to_owned());
        out.push("    }".to_owned());
        out.push(String::new());

        for (path, path_item) in self.paths()? {
            let path = value_string(path);
            let Some(path_item) = path_item.as_mapping() else {
                continue;
            };
            for method in HTTP_METHODS {
                let Some(operation) = mapping_get(path_item, method).and_then(Value::as_mapping)
                else {
                    continue;
                };
                if mapping_get(operation, "operationId").is_none() {
                    continue;
                }
                out.push(self.emit_typed_method(&path, method, operation)?);
                out.push(String::new());
            }
        }

        out.push("}".to_owned());
        Ok(out.join("\n"))
    }

    fn schemas(&self) -> Result<&Mapping> {
        self.mapping_path(&["components", "schemas"])
    }

    fn parameters(&self) -> Result<&Mapping> {
        self.mapping_path(&["components", "parameters"])
    }

    fn paths(&self) -> Result<&Mapping> {
        self.mapping_path(&["paths"])
    }

    fn mapping_path(&self, path: &[&str]) -> Result<&Mapping> {
        let mut value = self.spec;
        for segment in path {
            value = mapping_get(value.as_mapping().ok_or("expected YAML mapping")?, segment)
                .ok_or_else(|| format!("missing YAML key `{segment}`"))?;
        }
        value
            .as_mapping()
            .ok_or_else(|| format!("expected YAML mapping at `{}`", path.join(".")))
            .map_err(Into::into)
    }

    fn emit_schema(&self, name: &Value, schema: &Value) -> String {
        let name = value_string(name);
        let type_name = rust_type_name(&name);

        if enum_schema(schema) {
            let mut lines = Vec::new();
            lines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]".to_owned());
            lines.push(format!("pub enum {type_name} {{"));
            let mut seen = BTreeSet::new();
            for value in mapping_get(schema.as_mapping().expect("enum schema"), "enum")
                .and_then(Value::as_sequence)
                .into_iter()
                .flatten()
            {
                let raw = value.as_str().unwrap_or_default();
                let base = rust_enum_variant(raw);
                let mut variant = base.clone();
                let mut suffix = 2;
                while seen.contains(&variant) {
                    variant = format!("{base}{suffix}");
                    suffix += 1;
                }
                seen.insert(variant.clone());
                lines.push(format!("    #[serde(rename = {raw:?})]"));
                lines.push(format!("    {variant},"));
            }
            lines.push("}".to_owned());
            return lines.join("\n");
        }

        let Some(schema_map) = schema.as_mapping() else {
            return format!("pub type {type_name} = {};", schema_type(schema));
        };

        if value_as_str(mapping_get(schema_map, "type")) == Some("object")
            && mapping_get(schema_map, "properties")
                .and_then(Value::as_mapping)
                .is_some_and(|properties| !properties.is_empty())
        {
            let required = mapping_get(schema_map, "required")
                .and_then(Value::as_sequence)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect::<BTreeSet<_>>();
            let properties = mapping_get(schema_map, "properties")
                .and_then(Value::as_mapping)
                .expect("checked properties");

            let mut lines = Vec::new();
            lines.push(
                "#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]"
                    .to_owned(),
            );
            lines.push(format!("pub struct {type_name} {{"));
            for (prop_name, prop_schema) in properties {
                let prop_name = value_string(prop_name);
                let field_name = snake_name(&prop_name);
                let base_type = schema_type(prop_schema);
                let nullable = prop_schema
                    .as_mapping()
                    .and_then(|map| mapping_get(map, "nullable"))
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let optional = !required.contains(prop_name.as_str()) || nullable;
                let rust_type = if optional {
                    format!("Option<{base_type}>")
                } else {
                    base_type
                };

                lines.push(format!("    #[serde(rename = {prop_name:?})]"));
                if optional {
                    lines.push(
                        "    #[serde(default, skip_serializing_if = \"Option::is_none\")]"
                            .to_owned(),
                    );
                }
                lines.push(format!("    pub {field_name}: {rust_type},"));
            }
            lines.push("    #[serde(flatten)]".to_owned());
            lines.push(
                "    pub extra: std::collections::BTreeMap<String, serde_json::Value>,".to_owned(),
            );
            lines.push("}".to_owned());

            if type_name.ends_with("Request") {
                lines.push(String::new());
                lines.push(self.emit_request_builder(&type_name, properties, &required));
            }

            lines.join("\n")
        } else {
            format!("pub type {type_name} = {};", schema_type(schema))
        }
    }

    fn emit_request_builder(
        &self,
        type_name: &str,
        properties: &Mapping,
        required: &BTreeSet<&str>,
    ) -> String {
        let mut args = Vec::new();
        let mut assignments = Vec::new();

        for (prop_name, prop_schema) in properties {
            let prop_name = value_string(prop_name);
            if !required.contains(prop_name.as_str()) {
                continue;
            }
            let field_name = snake_name(&prop_name);
            let rust_type = schema_type(prop_schema);
            let (arg, value) = setter_arg_and_value(&field_name, &rust_type);
            args.push(arg);
            assignments.push(format!("            {field_name}: {value},"));
        }

        for (prop_name, _prop_schema) in properties {
            let prop_name = value_string(prop_name);
            if required.contains(prop_name.as_str()) {
                continue;
            }
            assignments.push(format!("            {}: None,", snake_name(&prop_name)));
        }
        assignments.push("            extra: std::collections::BTreeMap::new(),".to_owned());

        let mut lines = Vec::new();
        lines.push(format!("impl {type_name} {{"));
        lines.push(format!("    pub fn new({}) -> Self {{", args.join(", ")));
        lines.push("        Self {".to_owned());
        lines.extend(assignments);
        lines.push("        }".to_owned());
        lines.push("    }".to_owned());

        for (prop_name, prop_schema) in properties {
            let prop_name = value_string(prop_name);
            if required.contains(prop_name.as_str()) {
                continue;
            }
            let field_name = snake_name(&prop_name);
            let rust_type = schema_type(prop_schema);
            let (arg, value) = setter_arg_and_value(&field_name, &rust_type);
            lines.push(String::new());
            lines.push(format!(
                "    pub fn {field_name}(mut self, {arg}) -> Self {{"
            ));
            lines.push(format!("        self.{field_name} = Some({value});"));
            lines.push("        self".to_owned());
            lines.push("    }".to_owned());
        }

        lines.push("}".to_owned());
        lines.join("\n")
    }

    fn emit_typed_method(&self, path: &str, method: &str, operation: &Mapping) -> Result<String> {
        let operation_id = value_string(
            mapping_get(operation, "operationId").ok_or("operation missing operationId")?,
        );
        let method_name = snake_name(&operation_id);
        let path_params = path_params(path);
        let query_params = self.operation_query_params(operation)?;
        let has_query = !query_params.is_empty();
        let has_body = operation.contains_key(Value::String("requestBody".to_owned()));
        let query_type = if has_query {
            format!("crate::params::{}", self.operation_params_type(operation)?)
        } else {
            "()".to_owned()
        };
        let body_type = if has_body {
            self.operation_request_body_type(operation)
        } else {
            "()".to_owned()
        };
        let response_type = self
            .operation_success_schema(operation)
            .map(schema_type)
            .unwrap_or_else(|| "EmptyResponse".to_owned());

        let mut args = Vec::new();
        for param in &path_params {
            args.push(format!("{}: impl AsRef<str>", snake_name(param)));
        }
        if has_query {
            args.push(format!("params: &{query_type}"));
        }
        if has_body {
            args.push(format!("body: &{body_type}"));
        }

        let mut lines = String::new();
        let sig_args = if args.is_empty() {
            String::new()
        } else {
            format!(", {}", args.join(", "))
        };
        writeln!(
            lines,
            "    pub async fn {method_name}(&self{sig_args}) -> crate::Result<{response_type}>"
        )?;
        writeln!(lines, "    {{")?;
        writeln!(lines, "        let path = {};", path_expression(path))?;
        lines.push_str(&request_call(
            &response_type,
            method,
            has_query,
            has_body,
            &query_type,
            &body_type,
        ));
        writeln!(lines)?;
        write!(lines, "    }}")?;
        Ok(lines)
    }

    fn emit_params_struct(&self, operation: &Mapping) -> Result<Option<String>> {
        let params = self.operation_query_params(operation)?;
        if params.is_empty() {
            return Ok(None);
        }
        let type_name = self.operation_params_type(operation)?;
        let mut lines = Vec::new();
        lines.push(
            "#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]"
                .to_owned(),
        );
        lines.push(format!("pub struct {type_name} {{"));

        for param in &params {
            let param_name =
                value_string(mapping_get(param, "name").ok_or("parameter missing name")?);
            let field_name = snake_name(&param_name);
            let rust_type = parameter_type(param);
            lines.push(format!("    #[serde(rename = {param_name:?})]"));
            lines.push(
                "    #[serde(default, skip_serializing_if = \"Option::is_none\")]".to_owned(),
            );
            lines.push(format!("    pub {field_name}: Option<{rust_type}>,"));
        }
        lines.push("}".to_owned());
        lines.push(String::new());
        lines.push(format!("impl {type_name} {{"));
        lines.push("    pub fn new() -> Self {".to_owned());
        lines.push("        Self::default()".to_owned());
        lines.push("    }".to_owned());

        for param in &params {
            let param_name =
                value_string(mapping_get(param, "name").ok_or("parameter missing name")?);
            let field_name = snake_name(&param_name);
            let rust_type = parameter_type(param);
            let (arg, value) = setter_arg_and_value(&field_name, &rust_type);
            lines.push(String::new());
            lines.push(format!(
                "    pub fn {field_name}(mut self, {arg}) -> Self {{"
            ));
            lines.push(format!("        self.{field_name} = Some({value});"));
            lines.push("        self".to_owned());
            lines.push("    }".to_owned());
        }
        lines.push("}".to_owned());
        Ok(Some(lines.join("\n")))
    }

    fn operation_query_params<'a>(&'a self, operation: &'a Mapping) -> Result<Vec<&'a Mapping>> {
        let mut params = Vec::new();
        for param in mapping_get(operation, "parameters")
            .and_then(Value::as_sequence)
            .into_iter()
            .flatten()
        {
            let param = self.resolve_parameter(param)?;
            if value_as_str(mapping_get(param, "in")) == Some("query") {
                params.push(param);
            }
        }
        Ok(params)
    }

    fn resolve_parameter<'a>(&'a self, parameter: &'a Value) -> Result<&'a Mapping> {
        if let Some(reference) = parameter
            .as_mapping()
            .and_then(|map| mapping_get(map, "$ref"))
            .and_then(Value::as_str)
        {
            let name = reference
                .rsplit('/')
                .next()
                .ok_or("invalid parameter reference")?;
            return mapping_get(self.parameters()?, name)
                .and_then(Value::as_mapping)
                .ok_or_else(|| format!("unknown parameter reference `{name}`").into());
        }

        parameter
            .as_mapping()
            .ok_or_else(|| "parameter must be mapping".into())
    }

    fn operation_params_type(&self, operation: &Mapping) -> Result<String> {
        let operation_id = value_string(
            mapping_get(operation, "operationId").ok_or("operation missing operationId")?,
        );
        Ok(format!("{}Params", rust_type_name(&operation_id)))
    }

    fn operation_request_body_type(&self, operation: &Mapping) -> String {
        mapping_get(operation, "requestBody")
            .and_then(Value::as_mapping)
            .and_then(|body| mapping_get(body, "content"))
            .and_then(Value::as_mapping)
            .and_then(|content| mapping_get(content, "application/json"))
            .and_then(Value::as_mapping)
            .and_then(|json| mapping_get(json, "schema"))
            .map(schema_type)
            .unwrap_or_else(|| "serde_json::Value".to_owned())
    }

    fn operation_success_schema<'a>(&self, operation: &'a Mapping) -> Option<&'a Value> {
        let responses = mapping_get(operation, "responses")?.as_mapping()?;
        for status in ["200", "201", "202", "204"] {
            if let Some(schema) = mapping_get(responses, status)
                .and_then(Value::as_mapping)
                .and_then(|response| mapping_get(response, "content"))
                .and_then(Value::as_mapping)
                .and_then(|content| mapping_get(content, "application/json"))
                .and_then(Value::as_mapping)
                .and_then(|json| mapping_get(json, "schema"))
            {
                return Some(schema);
            }
        }
        None
    }
}

fn schema_type(schema: &Value) -> String {
    let Some(schema_map) = schema.as_mapping() else {
        return "serde_json::Value".to_owned();
    };

    if let Some(all_of) = mapping_get(schema_map, "allOf").and_then(Value::as_sequence)
        && all_of.len() == 1
    {
        return schema_type(&all_of[0]);
    }

    if let Some(reference) = mapping_get(schema_map, "$ref").and_then(Value::as_str) {
        return ref_name(reference);
    }

    if mapping_get(schema_map, "oneOf").is_some() || mapping_get(schema_map, "anyOf").is_some() {
        return "serde_json::Value".to_owned();
    }

    if mapping_get(schema_map, "enum").is_some() {
        return if value_as_str(mapping_get(schema_map, "type")) == Some("integer") {
            "i64".to_owned()
        } else {
            "String".to_owned()
        };
    }

    match value_as_str(mapping_get(schema_map, "type")) {
        Some("string") => "String".to_owned(),
        Some("integer") => "i64".to_owned(),
        Some("number") => "f64".to_owned(),
        Some("boolean") => "bool".to_owned(),
        Some("array") => {
            let item_type = mapping_get(schema_map, "items")
                .map(schema_type)
                .unwrap_or_else(|| "serde_json::Value".to_owned());
            format!("Vec<{item_type}>")
        }
        Some("object") => {
            if let Some(additional) = mapping_get(schema_map, "additionalProperties")
                && !matches!(additional, Value::Bool(false))
            {
                let inner = if matches!(additional, Value::Bool(true)) {
                    "serde_json::Value".to_owned()
                } else {
                    schema_type(additional)
                };
                return format!("std::collections::BTreeMap<String, {inner}>");
            }

            if mapping_get(schema_map, "properties").is_some() {
                "serde_json::Value".to_owned()
            } else {
                "std::collections::BTreeMap<String, serde_json::Value>".to_owned()
            }
        }
        _ => "serde_json::Value".to_owned(),
    }
}

fn parameter_type(parameter: &Mapping) -> String {
    mapping_get(parameter, "schema")
        .map(schema_type)
        .unwrap_or_else(|| "serde_json::Value".to_owned())
}

fn request_call(
    response_type: &str,
    method: &str,
    has_query: bool,
    has_body: bool,
    query_type: &str,
    body_type: &str,
) -> String {
    let query_arg = if has_query {
        "Some(params)"
    } else {
        "None::<&()>"
    };
    let body_arg = if has_body {
        "Some(body)"
    } else {
        "None::<&()>"
    };
    let query_type = if has_query { query_type } else { "()" };
    let body_type = if has_body { body_type } else { "()" };
    format!(
        "        self.client\n            .request_json::<{response_type}, {query_type}, {body_type}>(\n                reqwest::Method::{},\n                path.as_ref(),\n                {query_arg},\n                {body_arg},\n            )\n            .await",
        method.to_ascii_uppercase()
    )
}

fn setter_arg_and_value(field_name: &str, rust_type: &str) -> (String, String) {
    if STRING_LIKE_TYPES.contains(&rust_type) {
        (
            format!("{field_name}: impl Into<String>"),
            format!("{field_name}.into()"),
        )
    } else if rust_type.starts_with("Vec<String>") {
        (
            format!("{field_name}: impl IntoIterator<Item = impl Into<String>>"),
            format!("{field_name}.into_iter().map(Into::into).collect()"),
        )
    } else {
        (format!("{field_name}: {rust_type}"), field_name.to_owned())
    }
}

fn path_expression(path: &str) -> String {
    let params = path_params(path);
    if params.is_empty() {
        format!("{path:?}")
    } else {
        let mut format_string = path.to_owned();
        for param in &params {
            format_string = format_string.replace(&format!("{{{param}}}"), "{}");
        }
        let args = params
            .iter()
            .map(|param| format!("crate::rest::enc({})", snake_name(param)))
            .collect::<Vec<_>>();
        format!("format!({format_string:?}, {})", args.join(", "))
    }
}

fn path_params(path: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut rest = path;
    while let Some(start) = rest.find('{') {
        let after_start = &rest[start + 1..];
        let Some(end) = after_start.find('}') else {
            break;
        };
        params.push(after_start[..end].to_owned());
        rest = &after_start[end + 1..];
    }
    params
}

fn enum_schema(schema: &Value) -> bool {
    let Some(schema_map) = schema.as_mapping() else {
        return false;
    };
    mapping_get(schema_map, "enum")
        .and_then(Value::as_sequence)
        .is_some_and(|values| {
            let string_enum = value_as_str(mapping_get(schema_map, "type"))
                .is_none_or(|schema_type| schema_type == "string");
            string_enum && values.iter().all(|value| value.as_str().is_some())
        })
}

fn ref_name(reference: &str) -> String {
    rust_type_name(reference.rsplit('/').next().unwrap_or(reference))
}

fn rust_enum_variant(value: &str) -> String {
    let variant = rust_type_name(value);
    if KEYWORDS.contains(&variant.to_ascii_lowercase().as_str()) {
        format!("Type{variant}")
    } else {
        variant
    }
}

fn rust_type_name(name: &str) -> String {
    let parts = name
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<String>();
    let value = if parts.is_empty() {
        "GeneratedType".to_owned()
    } else {
        parts
    };
    if value.starts_with(|ch: char| ch.is_ascii_digit()) {
        format!("Type{value}")
    } else {
        value
    }
}

fn snake_name(name: &str) -> String {
    let chars = name.chars().collect::<Vec<_>>();
    let mut out = String::new();

    for (idx, ch) in chars.iter().enumerate() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && idx > 0 {
                let prev = chars[idx - 1];
                let next = chars.get(idx + 1).copied();
                if prev.is_ascii_lowercase()
                    || prev.is_ascii_digit()
                    || (prev.is_ascii_uppercase()
                        && next.is_some_and(|next| next.is_ascii_lowercase()))
                {
                    out.push('_');
                }
            }
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }

    let mut value = out.trim_matches('_').to_owned();
    if value.is_empty() {
        value = "field".to_owned();
    }
    if value.starts_with(|ch: char| ch.is_ascii_digit()) {
        value.insert(0, '_');
    }
    if KEYWORDS.contains(&value.as_str()) {
        value.push('_');
    }
    value
}

fn mapping_get<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a Value> {
    mapping.get(Value::String(key.to_owned()))
}

fn value_as_str(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str)
}

fn value_string(value: &Value) -> String {
    value.as_str().unwrap_or_default().to_owned()
}
