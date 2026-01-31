use tree_sitter::Parser;

use super::groovy;
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

pub fn parse_build_gradle(content: &str) -> Option<AllayDsl> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_groovy::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(content, None)?;

    let mut dsl = AllayDsl::default();

    groovy::parse(&tree.root_node(), content, &mut dsl);

    if dsl.api.is_some() || dsl.plugin.is_some() || dsl.has_allay_dependency {
        Some(dsl)
    } else {
        None
    }
}

/// Parse a Gradle settings file (settings.gradle or settings.gradle.kts) for
/// project metadata like `rootProject.name` and `version`.
/// Unlike `parse_build_gradle`/`parse_build_gradle_kts`, this does NOT filter
/// out results without allay dependencies since settings files never contain them.
pub fn parse_gradle_settings(path: &str, content: &str) -> AllayDsl {
    let mut parser = Parser::new();
    let lang = if path.ends_with(".gradle.kts") {
        tree_sitter_kotlin_ng::LANGUAGE.into()
    } else {
        tree_sitter_groovy::LANGUAGE.into()
    };
    if parser.set_language(&lang).is_err() {
        return AllayDsl::default();
    }
    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => return AllayDsl::default(),
    };
    let mut dsl = AllayDsl::default();
    if path.ends_with(".gradle.kts") {
        kts::parse(&tree.root_node(), content, &mut dsl);
    } else {
        groovy::parse(&tree.root_node(), content, &mut dsl);
    }
    dsl
}
