mod allay_block;
mod dependency;
#[cfg(test)]
mod tests;
mod util;

use crate::gradle::types::{AllayDsl, PluginDsl, VersionRef};
use tree_sitter::Node;

use allay_block::parse_allay_block;
use dependency::extract_allay_dependency;
use util::{extract_string, get_call_path, get_text, with_lambda};

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
            "call_expression" => {
                let call_path = get_call_path(&child, content);
                if call_path == "allay" {
                    let has_dep = dsl.has_allay_dependency;
                    let api_ref = std::mem::take(&mut dsl.api_version_ref);
                    let server_ref = std::mem::take(&mut dsl.server_version_ref);
                    let proj_ver = dsl.project_version.take();
                    let proj_desc = dsl.project_description.take();
                    *dsl = AllayDsl {
                        has_allay_dependency: has_dep,
                        api_version_ref: api_ref,
                        server_version_ref: server_ref,
                        project_version: proj_ver,
                        project_description: proj_desc,
                        ..Default::default()
                    };
                    with_lambda(&child, |lambda| {
                        parse_allay_block(lambda, content, dsl);
                    });
                } else if call_path == "allay.plugin" {
                    let mut plugin = PluginDsl::default();
                    with_lambda(&child, |lambda| {
                        allay_block::parse_plugin_block(lambda, content, &mut plugin);
                    });
                    dsl.plugin = Some(plugin);
                } else if let Some(dep_info) = extract_allay_dependency(&child, content) {
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
            "property_declaration" | "assignment" => {
                if let Some(ver) = try_parse_property_assignment(&child, content, "version") {
                    dsl.project_version = Some(ver);
                }
                if let Some(desc) = try_parse_property_assignment(&child, content, "description") {
                    dsl.project_description = Some(desc);
                }
            }
            _ => {}
        }

        parse(&child, content, dsl);
    }
}

fn try_parse_property_assignment(node: &Node, content: &str, property_name: &str) -> Option<String> {
    let mut cursor = node.walk();
    let mut found_property = false;
    let mut property_value = None;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "variable_declaration" => {
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if (inner.kind() == "identifier" || inner.kind() == "simple_identifier")
                        && get_text(&inner, content) == property_name
                    {
                        found_property = true;
                    }
                }
            }
            "identifier" | "simple_identifier" => {
                if get_text(&child, content) == property_name {
                    found_property = true;
                }
            }
            "directly_assignable_expression" => {
                let mut inner_cursor = child.walk();
                for inner in child.children(&mut inner_cursor) {
                    if (inner.kind() == "identifier" || inner.kind() == "simple_identifier")
                        && get_text(&inner, content) == property_name
                    {
                        found_property = true;
                    }
                }
            }
            "string_literal" | "line_string_literal" | "multiline_string_literal" => {
                property_value = extract_string(&child, content);
            }
            _ => {}
        }
    }

    if found_property { property_value } else { None }
}
