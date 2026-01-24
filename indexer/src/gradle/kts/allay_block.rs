use tree_sitter::Node;

use super::dependency::extract_dependencies;
use super::util::{collect_string_values, extract_value, get_call_name, get_text, with_lambda};
use crate::gradle::types::{AllayDsl, PluginDsl};

pub fn parse_allay_block(node: &Node, content: &str, dsl: &mut AllayDsl) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "statements" | "lambda_literal" => {
                parse_allay_block(&child, content, dsl);
            }
            "assignment" => {
                if let Some((key, value)) = parse_assignment(&child, content) {
                    match key.as_str() {
                        "api" => dsl.api = Some(value),
                        "apiOnly" => dsl.api_only = value.parse().ok(),
                        "server" => dsl.server = Some(value),
                        _ => {}
                    }
                }
            }
            "call_expression" => {
                if let Some(name) = get_call_name(&child, content)
                    && name == "plugin" {
                        let mut plugin = PluginDsl::default();
                        with_lambda(&child, |lambda| {
                            parse_plugin_block(lambda, content, &mut plugin);
                        });
                        dsl.plugin = Some(plugin);
                    }
            }
            _ => {}
        }
    }
}

pub fn parse_plugin_block(node: &Node, content: &str, plugin: &mut PluginDsl) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "statements" | "lambda_literal" => {
                parse_plugin_block(&child, content, plugin);
            }
            "assignment" => {
                if let Some((key, value)) = parse_assignment(&child, content) {
                    match key.as_str() {
                        "entrance" => plugin.entrance = Some(value),
                        "name" => plugin.name = Some(value),
                        "version" => plugin.version = Some(value),
                        "description" => plugin.description = Some(value),
                        "website" => plugin.website = Some(value),
                        "apiVersion" | "api" => plugin.api_version = Some(value),
                        _ => {}
                    }
                }

                if let Some((key, values)) = parse_augmented_assignment(&child, content) {
                    match key.as_str() {
                        "authors" => plugin.authors.extend(values),
                        "dependencies" => {
                            plugin
                                .dependencies
                                .extend(extract_dependencies(&child, content));
                        }
                        _ => {}
                    }
                }
            }
            "call_expression" => {
                if let Some(name) = get_call_name(&child, content)
                    && name == "dependencies" {
                        plugin
                            .dependencies
                            .extend(extract_dependencies(&child, content));
                    }
            }
            _ => {}
        }
    }
}

fn parse_assignment(node: &Node, content: &str) -> Option<(String, String)> {
    let mut cursor = node.walk();
    let mut lhs = None;
    let mut rhs = None;
    let mut is_simple_assign = false;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "directly_assignable_expression" => {
                lhs = Some(get_text(&child, content));
                is_simple_assign = true;
            }
            "identifier" | "simple_identifier" if lhs.is_none() => {
                lhs = Some(get_text(&child, content));
            }
            "=" => {
                is_simple_assign = true;
            }
            "+=" => {
                return None;
            }
            _ if is_simple_assign && rhs.is_none() => {
                rhs = extract_value(&child, content);
            }
            _ => {}
        }
    }

    match (lhs, rhs) {
        (Some(k), Some(v)) => Some((k, v)),
        _ => None,
    }
}

fn parse_augmented_assignment(node: &Node, content: &str) -> Option<(String, Vec<String>)> {
    let mut cursor = node.walk();
    let mut lhs = None;
    let mut values = Vec::new();
    let mut found_plus_eq = false;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "directly_assignable_expression" | "identifier" | "simple_identifier"
                if !found_plus_eq =>
            {
                lhs = Some(get_text(&child, content));
            }
            "+=" => {
                found_plus_eq = true;
            }
            _ if found_plus_eq => {
                values.extend(collect_string_values(&child, content));
            }
            _ => {}
        }
    }

    if let Some(k) = lhs
        && found_plus_eq {
            return Some((k, values));
        }
    None
}
