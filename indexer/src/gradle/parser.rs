use tree_sitter::Parser;

use super::kts;
use super::types::AllayDsl;

pub fn parse_build_gradle_kts(content: &str) -> Option<AllayDsl> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_kotlin_ng::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(content, None)?;

    let mut dsl = AllayDsl::default();

    kts::parse(&tree.root_node(), content, &mut dsl);

    if dsl.api.is_some() || dsl.plugin.is_some() || dsl.has_allay_dependency {
        Some(dsl)
    } else {
        None
    }
}
