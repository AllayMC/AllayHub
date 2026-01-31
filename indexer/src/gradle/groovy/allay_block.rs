use tree_sitter::Node;

use super::dependency::extract_dependencies;
use super::util::{
    collect_string_values, extract_value, get_assignment_lhs_name, get_call_name, get_text,
    with_closure,
};
use crate::gradle::types::{AllayDsl, PluginDsl};

pub fn parse_allay_block(node: &Node, content: &str, dsl: &mut AllayDsl) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "closure" => {
                parse_allay_block(&child, content, dsl);
            }
            "expression_statement" => {
                let mut inner = child.walk();
                for inner_child in child.children(&mut inner) {
                    handle_allay_child(&inner_child, content, dsl);
                }
            }
            "assignment_expression" => {
                handle_allay_assignment(&child, content, dsl);
            }
            "method_invocation" => {
                handle_allay_method(&child, content, dsl);
            }
            "juxt_function_call" => {
                handle_allay_juxt(&child, content, dsl);
            }
            _ => {}
        }
    }
}

fn handle_allay_child(node: &Node, content: &str, dsl: &mut AllayDsl) {
    match node.kind() {
        "assignment_expression" => {
            handle_allay_assignment(node, content, dsl);
        }
        "method_invocation" => {
            handle_allay_method(node, content, dsl);
        }
        "juxt_function_call" => {
            handle_allay_juxt(node, content, dsl);
        }
        _ => {}
    }
}

fn handle_allay_assignment(node: &Node, content: &str, dsl: &mut AllayDsl) {
    if let Some((key, op, value_node)) = parse_assignment_parts(node, content) {
        if op == "=" {
            if let Some(value) = extract_value(&value_node, content) {
                match key.as_str() {
                    "api" => dsl.api = Some(value),
                    "apiOnly" => dsl.api_only = value.parse().ok(),
                    "server" => dsl.server = Some(value),
                    _ => {}
                }
            }
        }
    }
}

fn handle_allay_method(node: &Node, content: &str, dsl: &mut AllayDsl) {
    if let Some(name) = get_call_name(node, content)
        && name == "plugin"
    {
        let mut plugin = PluginDsl::default();
        with_closure(node, |closure| {
            parse_plugin_block(closure, content, &mut plugin);
        });
        dsl.plugin = Some(plugin);
    }
}

fn handle_allay_juxt(node: &Node, content: &str, dsl: &mut AllayDsl) {
    if let Some((name, value)) = parse_juxt_string_value(node, content) {
        match name.as_str() {
            "api" => dsl.api = Some(value),
            "server" => dsl.server = Some(value),
            _ => {}
        }
    }
}

pub fn parse_plugin_block(node: &Node, content: &str, plugin: &mut PluginDsl) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "closure" => {
                parse_plugin_block(&child, content, plugin);
            }
            "expression_statement" => {
                let mut inner = child.walk();
                for inner_child in child.children(&mut inner) {
                    handle_plugin_child(&inner_child, content, plugin);
                }
            }
            "assignment_expression" => {
                handle_plugin_assignment(&child, content, plugin);
            }
            "method_invocation" => {
                handle_plugin_method(&child, content, plugin);
            }
            "juxt_function_call" => {
                handle_plugin_juxt(&child, content, plugin);
            }
            _ => {}
        }
    }
}

fn handle_plugin_child(node: &Node, content: &str, plugin: &mut PluginDsl) {
    match node.kind() {
        "assignment_expression" => {
            handle_plugin_assignment(node, content, plugin);
        }
        "method_invocation" => {
            handle_plugin_method(node, content, plugin);
        }
        "juxt_function_call" => {
            handle_plugin_juxt(node, content, plugin);
        }
        _ => {}
    }
}

fn handle_plugin_assignment(node: &Node, content: &str, plugin: &mut PluginDsl) {
    if let Some((key, op, value_node)) = parse_assignment_parts(node, content) {
        if op == "=" {
            // Handle simple value assignment
            if let Some(value) = extract_value(&value_node, content) {
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

            // Handle list assignment: `authors = ["a", "b"]` or `it.authors = ["Luck"]`
            let values = collect_string_values(&value_node, content);
            if !values.is_empty() {
                match key.as_str() {
                    "authors" => plugin.authors.extend(values),
                    _ => {}
                }
            }
        } else if op == "+=" {
            match key.as_str() {
                "authors" => {
                    let values = collect_string_values(&value_node, content);
                    plugin.authors.extend(values);
                }
                "dependencies" => {
                    // Pass the whole assignment node so extract_dependencies
                    // can recurse and find the dependency() call
                    plugin
                        .dependencies
                        .extend(extract_dependencies(node, content));
                }
                _ => {}
            }
        }
    }
}

fn handle_plugin_method(node: &Node, content: &str, plugin: &mut PluginDsl) {
    if let Some(name) = get_call_name(node, content)
        && name == "dependencies"
    {
        with_closure(node, |closure| {
            plugin
                .dependencies
                .extend(extract_dependencies(closure, content));
        });
    }
}

fn handle_plugin_juxt(node: &Node, content: &str, plugin: &mut PluginDsl) {
    if let Some((name, value)) = parse_juxt_string_value(node, content) {
        match name.as_str() {
            "entrance" => plugin.entrance = Some(value),
            "name" => plugin.name = Some(value),
            "version" => plugin.version = Some(value),
            "description" => plugin.description = Some(value),
            "website" => plugin.website = Some(value),
            "apiVersion" | "api" => plugin.api_version = Some(value),
            _ => {}
        }
    }
}

/// Parse an assignment_expression into (lhs_name, operator, rhs_node).
/// Handles both `name = value` and `it.name = value`.
fn parse_assignment_parts<'a>(
    node: &'a Node,
    content: &str,
) -> Option<(String, String, Node<'a>)> {
    let mut cursor = node.walk();
    let mut lhs: Option<String> = None;
    let mut op: Option<String> = None;
    let mut rhs: Option<Node<'a>> = None;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" | "field_access" if lhs.is_none() => {
                lhs = get_assignment_lhs_name(&child, content);
            }
            "=" | "+=" => {
                op = Some(get_text(&child, content));
            }
            _ if op.is_some() && rhs.is_none() => {
                rhs = Some(child);
            }
            _ => {}
        }
    }

    match (lhs, op, rhs) {
        (Some(l), Some(o), Some(r)) => Some((l, o, r)),
        _ => None,
    }
}

/// Parse a juxt_function_call like `name "value"` into (name, value).
fn parse_juxt_string_value(node: &Node, content: &str) -> Option<(String, String)> {
    let mut cursor = node.walk();
    let mut name = None;
    let mut value = None;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" if name.is_none() => {
                name = Some(get_text(&child, content));
            }
            "argument_list" => {
                let mut inner = child.walk();
                for arg in child.children(&mut inner) {
                    if value.is_none() {
                        value = extract_value(&arg, content);
                    }
                }
            }
            _ => {}
        }
    }

    match (name, value) {
        (Some(n), Some(v)) => Some((n, v)),
        _ => None,
    }
}
