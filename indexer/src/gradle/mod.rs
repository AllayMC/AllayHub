mod kts;
mod parser;
mod types;

pub use parser::parse_build_gradle_kts;
pub use types::{AllayDsl, GradleDependency, PluginDsl, PluginJson, VersionRef, parse_plugin_json};
