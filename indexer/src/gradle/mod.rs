mod groovy;
mod kts;
mod parser;
mod types;

pub use parser::{parse_build_gradle, parse_build_gradle_kts, parse_gradle_settings};
pub use types::{AllayDsl, GradleDependency, PluginDsl, PluginJson, VersionRef, parse_plugin_json};
