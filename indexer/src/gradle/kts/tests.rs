use crate::gradle::parse_build_gradle_kts;

mod allay_block {
    use super::*;

    #[test]
    fn test_parse_kotlin_dsl() {
        let content = r#"allay {
    api = "0.23.0-SNAPSHOT"
    apiOnly = false
    server = "0.1.0"
    plugin {
        entrance = ".AllayNPC"
        name = "AllayNPC"
        version = "1.0.0"
        description = "NPC plugin"
        authors += "daoge_cmd"
        website = "https://github.com/smartcmd/AllayNPC"
        apiVersion = ">=0.23.0"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.23.0-SNAPSHOT".to_string()));
        assert_eq!(dsl.api_only, Some(false));
        assert_eq!(dsl.server, Some("0.1.0".to_string()));

        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.entrance, Some(".AllayNPC".to_string()));
        assert_eq!(plugin.name, Some("AllayNPC".to_string()));
        assert_eq!(plugin.version, Some("1.0.0".to_string()));
        assert_eq!(plugin.description, Some("NPC plugin".to_string()));
        assert_eq!(plugin.authors, vec!["daoge_cmd"]);
        assert_eq!(
            plugin.website,
            Some("https://github.com/smartcmd/AllayNPC".to_string())
        );
        assert_eq!(plugin.api_version, Some(">=0.23.0".to_string()));
    }

    #[test]
    fn test_huhobot_style() {
        let content = r#"allay {
    api = "0.17.0"
    apiOnly = true
    server = null
    plugin {
        entrance = ".allay.HuHoBotAllay"
        apiVersion = ">=0.17.0"
        name = "HuHoBot"
        authors += "HuoHuas001"
        website = "https://github.com/HuHoBot/KotlinMergeAdapter"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.17.0".to_string()));
        assert_eq!(dsl.api_only, Some(true));
        assert_eq!(dsl.server, None);

        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.entrance, Some(".allay.HuHoBotAllay".to_string()));
        assert_eq!(plugin.api_version, Some(">=0.17.0".to_string()));
        assert_eq!(plugin.name, Some("HuHoBot".to_string()));
        assert_eq!(plugin.authors, vec!["HuoHuas001"]);
    }

    #[test]
    fn test_minebuilders_short_form() {
        let content = r#"allay.plugin {
    name = "TestPlugin"
    entrance = ".test.TestPlugin"
    description = "Test plugin of AllayMC Experimental!"
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("TestPlugin".to_string()));
        assert_eq!(plugin.entrance, Some(".test.TestPlugin".to_string()));
    }

    #[test]
    fn test_only_api_no_plugin() {
        let content = r#"allay {
    api = "0.23.0"
    apiOnly = true
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.23.0".to_string()));
        assert_eq!(dsl.api_only, Some(true));
        assert!(dsl.plugin.is_none());
    }

    #[test]
    fn test_empty_plugin_block() {
        let content = r#"allay {
    api = "0.1.0"
    plugin { }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.1.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, None);
        assert!(plugin.authors.is_empty());
    }

    #[test]
    fn test_plugin_with_api_shorthand() {
        let content = r#"allay {
    plugin {
        name = "Test"
        api = ">=0.20.0"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.api_version, Some(">=0.20.0".to_string()));
    }

    #[test]
    fn test_multiple_allay_blocks_takes_last() {
        let content = r#"allay {
    api = "1.0.0"
    plugin {
        name = "First"
    }
}
allay {
    api = "2.0.0"
    plugin {
        name = "Second"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("2.0.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("Second".to_string()));
    }

    #[test]
    fn test_no_allay_block() {
        let content = r#"plugins {
    kotlin("jvm")
}
dependencies {
    implementation("something")
}"#;
        let dsl = parse_build_gradle_kts(content);
        assert!(dsl.is_none());
    }

    #[test]
    fn test_allay_inside_other_blocks() {
        let content = r#"plugins {
    kotlin("jvm")
}

allay {
    api = "0.1.0"
    plugin {
        name = "Test"
    }
}

dependencies {
    implementation("something")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.1.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("Test".to_string()));
    }
}

mod plugin_block {
    use super::*;

    #[test]
    fn test_parse_list_authors() {
        let content = r#"allay {
    plugin {
        name = "Test"
        authors += listOf("a", "b")
        authors += "c"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.authors, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_mixed_authors() {
        let content = r#"allay {
    plugin {
        name = "Test"
        authors += "single"
        authors += listOf("list1", "list2")
        authors += mutableListOf("mut1")
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.authors, vec!["single", "list1", "list2", "mut1"]);
    }

    #[test]
    fn test_parse_dependencies() {
        let content = r#"allay {
    plugin {
        name = "MyPlugin"
        dependencies += dependency("OtherPlugin")
        dependencies += dependency("AnotherPlugin", "1.0.0", optional = true)
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.dependencies.len(), 2);

        assert_eq!(plugin.dependencies[0].name, "OtherPlugin");
        assert_eq!(plugin.dependencies[0].version, None);
        assert!(!plugin.dependencies[0].optional);

        assert_eq!(plugin.dependencies[1].name, "AnotherPlugin");
        assert_eq!(plugin.dependencies[1].version, Some("1.0.0".to_string()));
        assert!(plugin.dependencies[1].optional);
    }

    #[test]
    fn test_dependencies_with_listof() {
        let content = r#"allay {
    plugin {
        name = "Test"
        dependencies += listOf(
            dependency("PluginA"),
            dependency("PluginB", "1.0.0")
        )
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.dependencies.len(), 2);
        assert_eq!(plugin.dependencies[0].name, "PluginA");
        assert_eq!(plugin.dependencies[1].name, "PluginB");
    }

    #[test]
    fn test_dependencies_block_style() {
        let content = r#"allay {
    plugin {
        name = "Test"
        dependencies {
            dependency("PluginA")
            dependency("PluginB", "1.0.0")
            dependency("PluginC", optional = true)
        }
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.dependencies.len(), 3);
        assert!(plugin.dependencies[2].optional);
    }

    #[test]
    fn test_dependency_with_only_named_name() {
        let content = r#"allay {
    plugin {
        name = "Test"
        dependencies += dependency(name = "OnlyName")
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.dependencies[0].name, "OnlyName");
    }

    #[test]
    fn test_trailing_commas() {
        let content = r#"allay {
    plugin {
        name = "Test"
        authors += listOf(
            "Author1",
            "Author2",
        )
        dependencies += listOf(
            dependency("Dep1"),
            dependency("Dep2"),
        )
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.authors, vec!["Author1", "Author2"]);
        assert_eq!(plugin.dependencies.len(), 2);
    }
}

mod legacy_dependency {
    use super::*;

    #[test]
    fn test_compile_only_named() {
        let content = r#"dependencies {
    compileOnly(group = "org.allaymc.allay", name = "api", version = "0.14.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.14.0".to_string()));
        assert!(dsl.plugin.is_none());
    }

    #[test]
    fn test_compile_only_api() {
        let content = r#"dependencies {
    compileOnlyApi("org.allaymc.allay:api:0.24.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.24.0".to_string()));
    }

    #[test]
    fn test_api_dependency() {
        let content = r#"dependencies {
    api("org.allaymc.allay:server:0.24.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api_only, Some(false));
        assert_eq!(dsl.server, Some("0.24.0".to_string()));
    }

    #[test]
    fn test_compile_only_string() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay:api:0.14.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.14.0".to_string()));
    }

    #[test]
    fn test_implementation_allay() {
        let content = r#"dependencies {
    implementation(group = "org.allaymc.allay", name = "api", version = "0.14.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.14.0".to_string()));
    }

    #[test]
    fn test_server_dependency() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay:server:0.15.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api_only, Some(false));
        assert_eq!(dsl.server, Some("0.15.0".to_string()));
    }

    #[test]
    fn test_server_dependency_named() {
        let content = r#"dependencies {
    implementation(group = "org.allaymc.allay", name = "server", version = "0.16.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api_only, Some(false));
        assert_eq!(dsl.server, Some("0.16.0".to_string()));
    }

    #[test]
    fn test_both_api_and_server() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay:api:0.14.0")
    compileOnly("org.allaymc.allay:server:0.15.0")
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.14.0".to_string()));
        assert_eq!(dsl.api_only, Some(false));
        assert_eq!(dsl.server, Some("0.15.0".to_string()));
    }

    #[test]
    fn test_multi_positional_with_variable_version() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay", "api", Versions.Allay.api)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("".to_string()));
    }

    #[test]
    fn test_multi_positional_server_with_variable_version() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay", "server", Versions.Allay.server)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api_only, Some(false));
        assert_eq!(dsl.server, Some("".to_string()));
    }
}

mod version_catalog {
    use super::*;
    use crate::gradle::VersionRef;

    #[test]
    fn test_libs_allay() {
        let content = r#"dependencies {
    compileOnly(rootProject.libs.allay)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert!(matches!(dsl.api_version_ref, VersionRef::VersionCatalog(_)));
    }

    #[test]
    fn test_libs_allay_api() {
        let content = r#"dependencies {
    compileOnly(libs.allay.api)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert!(matches!(dsl.api_version_ref, VersionRef::VersionCatalog(_)));
    }

    #[test]
    fn test_libs_allay_server() {
        let content = r#"dependencies {
    compileOnly(libs.allay.server)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api_only, Some(false));
        assert!(matches!(
            dsl.server_version_ref,
            VersionRef::VersionCatalog(_)
        ));
    }
}

mod variable_version {
    use super::*;
    use crate::gradle::VersionRef;

    #[test]
    fn test_variable_version() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay", "api", Versions.Allay.api)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert!(matches!(dsl.api_version_ref, VersionRef::Variable(_)));
        if let VersionRef::Variable(path) = &dsl.api_version_ref {
            assert!(path.contains("Versions"));
        }
    }

    #[test]
    fn test_variable_version_server() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay", "server", Config.allayVersion)
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api_only, Some(false));
        assert!(matches!(dsl.server_version_ref, VersionRef::Variable(_)));
    }
}

mod string_parsing {
    use super::*;

    #[test]
    fn test_escaped_strings() {
        let content = r#"allay {
    plugin {
        name = "Test\"Plugin"
        description = "Line1\nLine2"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("Test\"Plugin".to_string()));
        assert_eq!(plugin.description, Some("Line1\nLine2".to_string()));
    }

    #[test]
    fn test_multiline_string() {
        let content = r#"allay {
    plugin {
        name = "Test"
        description = """This is a
multiline
description"""
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(
            plugin.description,
            Some("This is a\nmultiline\ndescription".to_string())
        );
    }

    #[test]
    fn test_multiline_string_with_trim() {
        let content = r#"allay {
    plugin {
        name = "Test"
        description = """
            Line 1
            Line 2
        """.trimIndent()
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert!(plugin.description.is_some());
        assert!(plugin.description.unwrap().contains("Line 1"));
    }

    #[test]
    fn test_string_with_unicode() {
        let content = r#"allay {
    plugin {
        name = "中文插件"
        description = "这是一个测试 🎮"
        authors += "日本語"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("中文插件".to_string()));
        assert_eq!(plugin.description, Some("这是一个测试 🎮".to_string()));
        assert_eq!(plugin.authors, vec!["日本語"]);
    }

    #[test]
    fn test_empty_multiline_string() {
        let content = r#"allay {
    plugin {
        name = "Test"
        description = """"""
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.description, Some("".to_string()));
    }

    #[test]
    fn test_string_with_special_chars() {
        let content = r#"allay {
    plugin {
        name = "Test<Plugin>"
        description = "Contains 'quotes' and \"double quotes\""
        website = "https://example.com/path?query=1&other=2"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("Test<Plugin>".to_string()));
        assert_eq!(
            plugin.description,
            Some("Contains 'quotes' and \"double quotes\"".to_string())
        );
    }

    #[test]
    fn test_all_fields_empty_strings() {
        let content = r#"allay {
    api = ""
    plugin {
        name = ""
        entrance = ""
        description = ""
        website = ""
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("".to_string()));
    }
}

mod syntax {
    use super::*;

    #[test]
    fn test_with_comments() {
        let content = r#"allay {
    // This is a comment
    api = "0.1.0"
    /* Block comment */
    plugin {
        name = "Test" // inline comment
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.1.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("Test".to_string()));
    }

    #[test]
    fn test_extra_whitespace() {
        let content = r#"allay    {
    api    =    "0.1.0"
    plugin     {
        name     =     "Test"
    }
}"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.1.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("Test".to_string()));
    }

    #[test]
    fn test_single_line_format() {
        let content = r#"allay { api = "0.1.0"; plugin { name = "T" } }"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.api, Some("0.1.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("T".to_string()));
    }
}

mod project_version {
    use super::*;

    #[test]
    fn test_version_assignment() {
        let content = r#"
version = "1.2.3"

dependencies {
    compileOnlyApi("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.project_version, Some("1.2.3".to_string()));
    }

    #[test]
    fn test_version_val_declaration() {
        let content = r#"
val version = "2.0.0"

dependencies {
    compileOnlyApi("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.project_version, Some("2.0.0".to_string()));
    }
}

mod project_description {
    use super::*;

    #[test]
    fn test_description_assignment() {
        let content = r#"
description = "A test plugin for Allay"

dependencies {
    compileOnlyApi("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(
            dsl.project_description,
            Some("A test plugin for Allay".to_string())
        );
    }

    #[test]
    fn test_description_val_declaration() {
        let content = r#"
val description = "Project description here"

dependencies {
    compileOnlyApi("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(
            dsl.project_description,
            Some("Project description here".to_string())
        );
    }

    #[test]
    fn test_version_and_description_together() {
        let content = r#"
version = "1.0.0"
description = "My awesome plugin"

dependencies {
    compileOnlyApi("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle_kts(content).unwrap();
        assert_eq!(dsl.project_version, Some("1.0.0".to_string()));
        assert_eq!(
            dsl.project_description,
            Some("My awesome plugin".to_string())
        );
    }
}
