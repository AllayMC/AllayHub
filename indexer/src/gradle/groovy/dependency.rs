use tree_sitter::Node;

use super::util::{CallArg, collect_call_args, get_call_name, get_field_access_path};
use crate::gradle::types::{GradleDependency, VersionRef};

pub enum AllayDepInfo {
    Api(VersionRef),
    Server(VersionRef),
    Unknown,
}

const DEPENDENCY_CONFIGS: &[&str] = &["compileOnly", "compileOnlyApi", "implementation", "api"];

/// Try to extract Allay dependency info from a method_invocation or juxt_function_call.
pub fn extract_allay_dependency(node: &Node, content: &str) -> Option<AllayDepInfo> {
    let name = get_call_name(node, content)?;
    if !DEPENDENCY_CONFIGS.contains(&name.as_str()) {
        return None;
    }

    // Check for version catalog reference (libs.allay.api)
    if let Some(info) = check_version_catalog_ref(node, content) {
        return Some(info);
    }

    let args = collect_call_args(node, content);
    let mut has_allay_group = false;
    let mut artifact_name: Option<&str> = None;
    let mut version: Option<String> = None;
    let mut version_var: Option<String> = None;
    let mut positional_strings: Vec<String> = Vec::new();

    for arg in &args {
        match arg {
            CallArg::Named(k, v) => {
                if k == "group" && v == "org.allaymc.allay" {
                    has_allay_group = true;
                }
                if k == "name" && (v == "api" || v == "server") {
                    artifact_name = Some(v.as_str());
                }
                if k == "version" {
                    version = Some(v.clone());
                }
            }
            CallArg::Positional(v) => {
                if v.starts_with("org.allaymc.allay:") {
                    let parts: Vec<&str> = v.split(':').collect();
                    if parts.len() >= 2 {
                        let art = parts[1];
                        if art == "api" || art == "server" {
                            artifact_name =
                                Some(if art == "api" { "api" } else { "server" });
                            has_allay_group = true;
                            if parts.len() >= 3 {
                                version = Some(parts[2].to_string());
                            }
                        }
                    }
                } else {
                    positional_strings.push(v.clone());
                }
            }
            CallArg::PositionalRef(path) => {
                version_var = Some(path.clone());
            }
        }
    }

    if !has_allay_group
        && positional_strings.len() >= 2
        && positional_strings[0] == "org.allaymc.allay"
    {
        has_allay_group = true;
        let art = &positional_strings[1];
        if art == "api" || art == "server" {
            artifact_name = Some(if art == "api" { "api" } else { "server" });
        }
        if positional_strings.len() >= 3 {
            version = Some(positional_strings[2].clone());
        }
    }

    if has_allay_group {
        let version_ref = if let Some(v) = version {
            VersionRef::Literal(v)
        } else if let Some(var) = version_var {
            VersionRef::Variable(var)
        } else {
            VersionRef::None
        };

        match artifact_name {
            Some("api") => Some(AllayDepInfo::Api(version_ref)),
            Some("server") => Some(AllayDepInfo::Server(version_ref)),
            _ => Some(AllayDepInfo::Unknown),
        }
    } else {
        None
    }
}

fn check_version_catalog_ref(node: &Node, content: &str) -> Option<AllayDepInfo> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "argument_list" => {
                if let Some(info) = check_version_catalog_ref(&child, content) {
                    return Some(info);
                }
            }
            "field_access" => {
                let path = get_field_access_path(&child, content);
                let path_lower = path.to_lowercase();
                if path_lower.contains("libs") && path_lower.contains("allay") {
                    let version_ref = VersionRef::VersionCatalog(path);
                    if path_lower.contains("server") {
                        return Some(AllayDepInfo::Server(version_ref));
                    }
                    return Some(AllayDepInfo::Api(version_ref));
                }
            }
            _ => {}
        }
    }

    None
}

/// Extract plugin dependencies from a `dependency(...)` call or a block.
pub fn extract_dependencies(node: &Node, content: &str) -> Vec<GradleDependency> {
    let mut deps = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "method_invocation" => {
                if let Some(name) = get_call_name(&child, content)
                    && name == "dependency"
                    && let Some(dep) = parse_dependency_call(&child, content)
                {
                    deps.push(dep);
                }
            }
            "juxt_function_call" => {
                if let Some(name) = get_call_name(&child, content)
                    && name == "dependency"
                    && let Some(dep) = parse_dependency_call(&child, content)
                {
                    deps.push(dep);
                }
            }
            _ => {
                deps.extend(extract_dependencies(&child, content));
            }
        }
    }

    deps
}

fn parse_dependency_call(node: &Node, content: &str) -> Option<GradleDependency> {
    let mut dep = GradleDependency::default();
    let args = collect_call_args(node, content);

    let mut positional_idx = 0;
    for arg in args {
        match arg {
            CallArg::Positional(v) => {
                if positional_idx == 0 {
                    dep.name = v;
                } else if positional_idx == 1 {
                    dep.version = Some(v);
                }
                positional_idx += 1;
            }
            CallArg::Named(k, v) => match k.as_str() {
                "name" => dep.name = v,
                "version" => dep.version = Some(v),
                "optional" => dep.optional = v == "true",
                _ => {}
            },
            CallArg::PositionalRef(_) => {
                positional_idx += 1;
            }
        }
    }

    if dep.name.is_empty() {
        None
    } else {
        Some(dep)
    }
}
