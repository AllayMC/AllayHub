mod allay_block;
mod dependency;
#[cfg(test)]
mod tests;
mod util;

use crate::gradle::types::{AllayDsl, PluginDsl, VersionRef};
use tree_sitter::Node;

use allay_block::parse_allay_block;
use dependency::extract_allay_dependency;
use util::{get_call_path, get_text, with_closure};

fn version_ref_to_string(version_ref: &VersionRef) -> String {
    match version_ref {
        VersionRef::Literal(v) => v.clone(),
        _ => String::new(),
    }
}

pub fn parse(node: &Node, content: &str, dsl: &mut AllayDsl) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "method_invocation" => {
                handle_call(&child, content, dsl);
            }
            "juxt_function_call" => {
                handle_call(&child, content, dsl);
            }
            "expression_statement" => {
                let mut inner = child.walk();
                for inner_child in child.children(&mut inner) {
                    match inner_child.kind() {
                        "method_invocation" | "juxt_function_call" => {
                            handle_call(&inner_child, content, dsl);
                        }
                        "assignment_expression" => {
                            try_parse_top_level_assignment(&inner_child, content, dsl);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        parse(&child, content, dsl);
    }
}

fn handle_call(node: &Node, content: &str, dsl: &mut AllayDsl) {
    let call_path = get_call_path(node, content);

    if call_path == "allay" {
        let has_dep = dsl.has_allay_dependency;
        let api_ref = std::mem::take(&mut dsl.api_version_ref);
        let server_ref = std::mem::take(&mut dsl.server_version_ref);
        let proj_name = dsl.project_name.take();
        let proj_ver = dsl.project_version.take();
        let proj_desc = dsl.project_description.take();
        *dsl = AllayDsl {
            has_allay_dependency: has_dep,
            api_version_ref: api_ref,
            server_version_ref: server_ref,
            project_name: proj_name,
            project_version: proj_ver,
            project_description: proj_desc,
            ..Default::default()
        };
        with_closure(node, |closure| {
            parse_allay_block(closure, content, dsl);
        });
    } else if call_path == "allay.plugin" {
        let mut plugin = PluginDsl::default();
        with_closure(node, |closure| {
            allay_block::parse_plugin_block(closure, content, &mut plugin);
        });
        dsl.plugin = Some(plugin);
    } else if let Some(dep_info) = extract_allay_dependency(node, content) {
        dsl.has_allay_dependency = true;
        match dep_info {
            dependency::AllayDepInfo::Api(version_ref) => {
                dsl.api = Some(version_ref_to_string(&version_ref));
                dsl.api_version_ref = version_ref;
            }
            dependency::AllayDepInfo::Server(version_ref) => {
                dsl.api_only = Some(false);
                dsl.server = Some(version_ref_to_string(&version_ref));
                dsl.server_version_ref = version_ref;
            }
            dependency::AllayDepInfo::Unknown => {}
        }
    }
}

fn try_parse_top_level_assignment(node: &Node, content: &str, dsl: &mut AllayDsl) {
    let mut cursor = node.walk();
    let mut lhs = None;
    let mut found_eq = false;
    let mut value = None;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" if !found_eq => {
                lhs = Some(get_text(&child, content));
            }
            "field_access" if !found_eq => {
                lhs = Some(util::get_field_access_path(&child, content));
            }
            "=" => {
                found_eq = true;
            }
            _ if found_eq && value.is_none() => {
                value = util::extract_value(&child, content);
            }
            _ => {}
        }
    }

    if let (Some(name), true, Some(val)) = (lhs, found_eq, value) {
        match name.as_str() {
            "version" => dsl.project_version = Some(val),
            "description" => dsl.project_description = Some(val),
            "rootProject.name" => dsl.project_name = Some(val),
            _ => {}
        }
    }
}
