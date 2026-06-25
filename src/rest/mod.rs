//! High-level REST namespace clients.
//!
//! These clients group generated OpenAPI operations by product area and keep normal SDK usage away
//! from raw paths and manual query encoding. Each method delegates to the typed operation layer in
//! [`crate::typed`].

pub mod account;
pub mod api_keys;
pub mod communications;
pub mod events;
pub mod exchange;
pub mod fcm;
pub mod historical;
pub mod incentive_programs;
pub mod live_data;
pub mod markets;
pub mod milestones;
pub mod multivariate;
pub mod order_groups;
pub mod orders;
pub mod portfolio;
pub mod search;
pub mod structured_targets;

pub(crate) fn enc(segment: impl AsRef<str>) -> String {
    urlencoding::encode(segment.as_ref()).into_owned()
}
