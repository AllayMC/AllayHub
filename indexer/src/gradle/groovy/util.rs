use tree_sitter::Node;

pub fn get_text(node: &Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

/// Get the simple name of a method_invocation or juxt_function_call.
pub fn get_call_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            return Some(get_text(&child, content));
        }
        if child.kind() == "field_access" {
            return get_last_identifier(&child, content);
        }
    }
    None
}

/// Get the full dotted path of a call. E.g., `allay.plugin { }` → "allay.plugin".
pub fn get_call_path(node: &Node, content: &str) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            return get_text(&child, content);
        }
        if child.kind() == "field_access" {
            return get_field_access_path(&child, content);
        }
    }
    String::new()
}

/// Recursively build a dotted path from nested field_access nodes.
/// `libs.allay.api` → "libs.allay.api"
pub fn get_field_access_path(node: &Node, content: &str) -> String {
    let mut parts = Vec::new();
    collect_field_access_parts(node, content, &mut parts);
    parts.join(".")
}

fn collect_field_access_parts(node: &Node, content: &str, parts: &mut Vec<String>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" => {
                parts.push(get_text(&child, content));
            }
            "field_access" => {
                collect_field_access_parts(&child, content, parts);
            }
            _ => {}
        }
    }
}

fn get_last_identifier(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    let mut last = None;
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            last = Some(get_text(&child, content));
        }
    }
    last
}

/// Find a closure body inside a method_invocation and apply `f` to it.
pub fn with_closure<F, R>(node: &Node, f: F) -> Option<R>
where
    F: FnOnce(&Node) -> R,
{
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "closure" {
            return Some(f(&child));
        }
    }
    None
}

/// Extract a string value from a string_literal or character_literal node.
/// Handles both double-quoted `"..."` and single-quoted `'...'` strings.
pub fn extract_string(node: &Node, content: &str) -> Option<String> {
    match node.kind() {
        "string_literal" => {
            // Try to find string_fragment child first
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "string_fragment"
                    || child.kind() == "multiline_string_fragment"
                {
                    return Some(get_text(&child, content));
                }
            }
            // Fallback: strip double quotes
            let text = get_text(node, content);
            strip_double_quotes(&text)
        }
        "character_literal" => {
            // Single-quoted string in Groovy: '...'
            let text = get_text(node, content);
            strip_single_quotes(&text)
        }
        _ => None,
    }
}

fn strip_double_quotes(text: &str) -> Option<String> {
    if text.starts_with("\"\"\"") && text.ends_with("\"\"\"") && text.len() >= 6 {
        return Some(text[3..text.len() - 3].to_string());
    }
    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
        return Some(unescape(&text[1..text.len() - 1]));
    }
    None
}

fn strip_single_quotes(text: &str) -> Option<String> {
    if text.starts_with("'''") && text.ends_with("'''") && text.len() >= 6 {
        return Some(text[3..text.len() - 3].to_string());
    }
    if text.starts_with('\'') && text.ends_with('\'') && text.len() >= 2 {
        return Some(text[1..text.len() - 1].to_string());
    }
    None
}

/// Extract a value from various expression types.
pub fn extract_value(node: &Node, content: &str) -> Option<String> {
    match node.kind() {
        "string_literal" | "character_literal" => extract_string(node, content),
        "decimal_integer_literal" | "decimal_floating_point_literal" => {
            Some(get_text(node, content))
        }
        "true" => Some("true".to_string()),
        "false" => Some("false".to_string()),
        "identifier" => {
            let text = get_text(node, content);
            if text == "true" || text == "false" {
                Some(text)
            } else {
                None
            }
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if let Some(v) = extract_value(&child, content) {
                    return Some(v);
                }
            }
            None
        }
    }
}

fn unescape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    result
}

pub enum CallArg {
    Positional(String),
    Named(String, String),
    PositionalRef(String),
}

/// Collect arguments from a method_invocation or juxt_function_call's argument_list.
pub fn collect_call_args(node: &Node, content: &str) -> Vec<CallArg> {
    let mut args = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "argument_list" => {
                args.extend(collect_args_from_list(&child, content));
            }
            _ => {}
        }
    }

    args
}

fn collect_args_from_list(node: &Node, content: &str) -> Vec<CallArg> {
    let mut args = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "map_item" => {
                // Named argument: `key: value`
                if let Some((key, val)) = parse_map_item(&child, content) {
                    args.push(CallArg::Named(key, val));
                }
            }
            "string_literal" | "character_literal" => {
                if let Some(s) = extract_string(&child, content) {
                    args.push(CallArg::Positional(s));
                }
            }
            "field_access" => {
                let path = get_field_access_path(&child, content);
                args.push(CallArg::PositionalRef(path));
            }
            "identifier" => {
                let text = get_text(&child, content);
                if text == "true" || text == "false" {
                    args.push(CallArg::Positional(text));
                } else {
                    args.push(CallArg::PositionalRef(text));
                }
            }
            _ => {
                if let Some(v) = extract_value(&child, content) {
                    args.push(CallArg::Positional(v));
                }
            }
        }
    }

    args
}

fn parse_map_item(node: &Node, content: &str) -> Option<(String, String)> {
    let mut cursor = node.walk();
    let mut key = None;
    let mut value = None;
    let mut found_colon = false;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" if !found_colon => {
                key = Some(get_text(&child, content));
            }
            ":" => {
                found_colon = true;
            }
            _ if found_colon && value.is_none() => {
                value = extract_value(&child, content);
            }
            _ => {}
        }
    }

    match (key, value) {
        (Some(k), Some(v)) => Some((k, v)),
        _ => None,
    }
}

/// Collect string values from various collection expressions.
/// Handles array_literal `[...]`, listOf(...), and single strings.
pub fn collect_string_values(node: &Node, content: &str) -> Vec<String> {
    let mut values = Vec::new();

    match node.kind() {
        "string_literal" | "character_literal" => {
            if let Some(s) = extract_string(node, content) {
                values.push(s);
            }
        }
        "array_literal" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "string_literal" || child.kind() == "character_literal" {
                    if let Some(s) = extract_string(&child, content) {
                        values.push(s);
                    }
                }
            }
        }
        "method_invocation" => {
            if let Some(name) = get_call_name(node, content)
                && (name == "listOf" || name == "mutableListOf")
            {
                values.extend(collect_call_string_args(node, content));
            }
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                values.extend(collect_string_values(&child, content));
            }
        }
    }

    values
}

fn collect_call_string_args(node: &Node, content: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "argument_list" => {
                let mut inner = child.walk();
                for arg in child.children(&mut inner) {
                    if arg.kind() == "string_literal" || arg.kind() == "character_literal" {
                        if let Some(s) = extract_string(&arg, content) {
                            values.push(s);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    values
}

/// Get the name from the LHS of an assignment, stripping `it.` prefix.
/// `it.name` → "name", `name` → "name"
pub fn get_assignment_lhs_name(node: &Node, content: &str) -> Option<String> {
    match node.kind() {
        "identifier" => Some(get_text(node, content)),
        "field_access" => {
            let path = get_field_access_path(node, content);
            // Strip `it.` prefix for Groovy closure delegate
            if let Some(stripped) = path.strip_prefix("it.") {
                Some(stripped.to_string())
            } else {
                Some(path)
            }
        }
        _ => None,
    }
}
