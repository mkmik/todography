//! Checks and queries the map of components in docs/map/. See README.md for
//! the format and the commands.

pub mod compute;
pub mod json;
pub mod label;
pub mod map;
pub mod query;
pub mod tui;

pub use map::Map;
