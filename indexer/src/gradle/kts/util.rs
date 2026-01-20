use tree_sitter::Node;

pub fn get_call_name(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "simple_identifier" {
            return Some(get_text(&child, content));
        }
        if child.kind() == "navigation_expression" {
            return get_last_identifier(&child, content);
        }
    }
    None
}

pub fn get_call_path(node: &Node, content: &str) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "simple_identifier" {
            return get_text(&child, content);
        }
        if child.kind() == "navigation_expression" {
            return get_navigation_path(&child, content);
        }
    }
    String::new()
}

pub fn get_navigation_path(node: &Node, content: &str) -> String {
    let mut parts = Vec::new();
    collect_nav_parts(node, content, &mut parts);
    parts.join(".")
}

fn collect_nav_parts(node: &Node, content: &str, parts: &mut Vec<String>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" | "simple_identifier" => {
                parts.push(get_text(&child, content));
            }
            "navigation_expression" => {
                collect_nav_parts(&child, content, parts);
            }
            _ => {}
        }
    }
}

fn get_last_identifier(node: &Node, content: &str) -> Option<String> {
    let mut cursor = node.walk();
    let mut last = None;
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" || child.kind() == "simple_identifier" {
            last = Some(get_text(&child, content));
        }
    }
    last
}

pub fn with_lambda<F, R>(node: &Node, f: F) -> Option<R>
where
    F: FnOnce(&Node) -> R,
{
    fn find_and_apply<F2, R2>(node: &Node, f: &mut Option<F2>) -> Option<R2>
    where
        F2: FnOnce(&Node) -> R2,
    {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "lambda_literal" | "annotated_lambda" => {
                    if let Some(func) = f.take() {
                        return Some(func(&child));
                    }
                }
                "call_suffix" => {
                    if let Some(r) = find_and_apply(&child, f) {
                        return Some(r);
                    }
                }
                _ => {}
            }
        }
        None
    }
    let mut f = Some(f);
    find_and_apply(node, &mut f)
}

pub fn get_text(node: &Node, content: &str) -> String {
    content[node.byte_range()].to_string()
}

pub fn extract_string(node: &Node, content: &str) -> Option<String> {
    let text = get_text(node, content);

    if text.starts_with("\"\"\"") && text.ends_with("\"\"\"") && text.len() >= 6 {
        return Some(text[3..text.len() - 3].to_string());
    }

    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
        return Some(unescape(&text[1..text.len() - 1]));
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "line_string_content" || child.kind() == "string_content" {
            return Some(get_text(&child, content));
        }
    }

    None
}

pub fn extract_value(node: &Node, content: &str) -> Option<String> {
    match node.kind() {
        "string_literal" | "line_string_literal" | "multiline_string_literal" => {
            extract_string(node, content)
        }
        "boolean_literal" | "integer_literal" | "real_literal" => Some(get_text(node, content)),
        "identifier" | "simple_identifier" => {
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

pub fn collect_call_args(node: &Node, content: &str) -> Vec<CallArg> {
    let mut args = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "call_suffix" | "value_arguments" => {
                args.extend(collect_call_args(&child, content));
            }
            "value_argument" => {
                if let Some(arg) = parse_value_argument(&child, content) {
                    args.push(arg);
                }
            }
            _ => {}
        }
    }

    args
}

fn parse_value_argument(node: &Node, content: &str) -> Option<CallArg> {
    let mut cursor = node.walk();
    let mut name = None;
    let mut value = None;
    let mut found_eq = false;
    let mut nav_ref = None;

    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" | "simple_identifier" => {
                let text = get_text(&child, content);
                if !found_eq {
                    name = Some(text);
                } else if text == "true" || text == "false" {
                    value = Some(text);
                }
            }
            "=" => {
                found_eq = true;
            }
            "string_literal" | "line_string_literal" | "multiline_string_literal" => {
                value = extract_string(&child, content);
            }
            "boolean_literal" => {
                value = Some(get_text(&child, content));
            }
            "navigation_expression" => {
                nav_ref = Some(get_navigation_path(&child, content));
            }
            _ => {
                if value.is_none() {
                    value = extract_value(&child, content);
                }
            }
        }
    }

    match (name, value, found_eq, nav_ref) {
        (Some(n), Some(v), true, _) => Some(CallArg::Named(n, v)),
        (_, Some(v), false, _) => Some(CallArg::Positional(v)),
        (Some(n), None, false, _) if n == "true" || n == "false" => Some(CallArg::Positional(n)),
        (_, None, false, Some(r)) => Some(CallArg::PositionalRef(r)),
        _ => None,
    }
}

pub fn collect_string_values(node: &Node, content: &str) -> Vec<String> {
    let mut values = Vec::new();

    match node.kind() {
        "string_literal" | "line_string_literal" | "multiline_string_literal" => {
            if let Some(s) = extract_string(node, content) {
                values.push(s);
            }
        }
        "call_expression" => {
            if let Some(name) = get_call_name(node, content) {
                if name == "listOf" || name == "mutableListOf" {
                    values.extend(collect_call_string_args(node, content));
                }
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
            "call_suffix" | "value_arguments" => {
                values.extend(collect_call_string_args(&child, content));
            }
            "value_argument" => {
                values.extend(collect_string_values(&child, content));
            }
            "string_literal" | "line_string_literal" | "multiline_string_literal" => {
                if let Some(s) = extract_string(&child, content) {
                    values.push(s);
                }
            }
            _ => {}
        }
    }

    values
}
