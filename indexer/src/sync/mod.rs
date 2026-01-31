pub(crate) mod builder;
pub mod discover;
pub mod update;

pub use discover::discover_new_plugins;
pub use update::{UpdateResult, update_existing_plugins};
