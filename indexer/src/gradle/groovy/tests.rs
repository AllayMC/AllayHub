use crate::gradle::parse_build_gradle;
use crate::gradle::types::VersionRef;

mod allay_block {
    use super::*;

    #[test]
    fn test_assignment_style() {
        let content = r#"allay {
    api = "0.23.0"
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
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.api, Some("0.23.0".to_string()));
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
    fn test_it_prefix_style() {
        let content = r#"allay {
    api = "0.17.0"
    plugin {
        it.entrance = "me.lucko.luckperms.allay.loader.AllayLoaderPlugin"
        it.name = "LuckPerms"
        it.description = "A permissions plugin."
        it.authors = ["Luck"]
        it.website = "https://luckperms.net"
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.api, Some("0.17.0".to_string()));

        let plugin = dsl.plugin.unwrap();
        assert_eq!(
            plugin.entrance,
            Some("me.lucko.luckperms.allay.loader.AllayLoaderPlugin".to_string())
        );
        assert_eq!(plugin.name, Some("LuckPerms".to_string()));
        assert_eq!(plugin.description, Some("A permissions plugin.".to_string()));
        assert_eq!(plugin.authors, vec!["Luck"]);
        assert_eq!(plugin.website, Some("https://luckperms.net".to_string()));
    }

    #[test]
    fn test_single_quote_strings() {
        let content = r#"allay {
    api = '0.23.0'
    plugin {
        name = 'MyPlugin'
        entrance = '.MyPlugin'
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.api, Some("0.23.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.name, Some("MyPlugin".to_string()));
        assert_eq!(plugin.entrance, Some(".MyPlugin".to_string()));
    }

    #[test]
    fn test_groovy_dsl_method_style() {
        let content = r#"allay {
    api "0.23.0"
    plugin {
        entrance ".AllayNPC"
        name "AllayNPC"
        version "1.0.0"
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.api, Some("0.23.0".to_string()));
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.entrance, Some(".AllayNPC".to_string()));
        assert_eq!(plugin.name, Some("AllayNPC".to_string()));
        assert_eq!(plugin.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_authors_array_literal() {
        let content = r#"allay {
    plugin {
        name = "Test"
        entrance = ".Test"
        authors = ["Author1", "Author2"]
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.authors, vec!["Author1", "Author2"]);
    }

    #[test]
    fn test_authors_augmented_assignment() {
        let content = r#"allay {
    plugin {
        name = "Test"
        entrance = ".Test"
        authors += "a"
        authors += ["b", "c"]
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.authors, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_dependencies_augmented() {
        let content = r#"allay {
    plugin {
        name = "Test"
        entrance = ".Test"
        dependencies += dependency("OtherPlugin")
        dependencies += dependency("AnotherPlugin", "1.0.0")
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.dependencies.len(), 2);
        assert_eq!(plugin.dependencies[0].name, "OtherPlugin");
        assert_eq!(plugin.dependencies[1].name, "AnotherPlugin");
        assert_eq!(
            plugin.dependencies[1].version,
            Some("1.0.0".to_string())
        );
    }

    #[test]
    fn test_dependencies_block() {
        let content = r#"allay {
    plugin {
        name = "Test"
        entrance = ".Test"
        dependencies {
            dependency("PluginA")
            dependency("PluginB", "1.0.0")
        }
    }
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        let plugin = dsl.plugin.unwrap();
        assert_eq!(plugin.dependencies.len(), 2);
        assert_eq!(plugin.dependencies[0].name, "PluginA");
        assert_eq!(plugin.dependencies[1].name, "PluginB");
    }
}

mod legacy_dependency {
    use super::*;

    #[test]
    fn test_juxt_compile_only() {
        let content = r#"dependencies {
    compileOnly "org.allaymc.allay:api:0.17.0"
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.17.0".to_string()));
        assert!(matches!(dsl.api_version_ref, VersionRef::Literal(v) if v == "0.17.0"));
    }

    #[test]
    fn test_parens_compile_only() {
        let content = r#"dependencies {
    compileOnly("org.allaymc.allay:api:0.24.0")
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.24.0".to_string()));
    }

    #[test]
    fn test_named_args_with_colon() {
        let content = r#"dependencies {
    compileOnly group: "org.allaymc.allay", name: "api", version: "0.14.0"
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.14.0".to_string()));
    }

    #[test]
    fn test_server_dependency() {
        let content = r#"dependencies {
    implementation("org.allaymc.allay:server:0.1.0")
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.server, Some("0.1.0".to_string()));
        assert_eq!(dsl.api_only, Some(false));
    }

    #[test]
    fn test_version_catalog_parens() {
        let content = r#"dependencies {
    compileOnly(libs.allay.api)
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert!(matches!(dsl.api_version_ref, VersionRef::VersionCatalog(_)));
    }
}

mod project_version {
    use super::*;

    #[test]
    fn test_double_quote_version() {
        let content = r#"
version = "1.2.3"
dependencies {
    compileOnly("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.project_version, Some("1.2.3".to_string()));
    }

    #[test]
    fn test_single_quote_version() {
        let content = r#"
version = '1.2.3'
dependencies {
    compileOnly("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.project_version, Some("1.2.3".to_string()));
    }

    #[test]
    fn test_description() {
        let content = r#"
description = "My plugin description"
dependencies {
    compileOnly("org.allaymc.allay:api:0.24.0")
}
"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(
            dsl.project_description,
            Some("My plugin description".to_string())
        );
    }
}

mod real_world {
    use super::*;

    #[test]
    fn test_luckperms_allay_build() {
        // Real content from AllayMC/LuckPerms allay/build.gradle
        let content = r#"plugins {
    alias(libs.plugins.shadow)
}

sourceCompatibility = 21
targetCompatibility = 21

repositories {
    maven { url 'https://central.sonatype.com/repository/maven-snapshots/' }
}

dependencies {
    implementation project(':common')
    compileOnly project(':common:loader-utils')

    compileOnly "org.allaymc.allay:api:0.17.0"
}

shadowJar {
    archiveFileName = 'luckperms-allay.jarinjar'
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert!(dsl.has_allay_dependency);
        assert_eq!(dsl.api, Some("0.17.0".to_string()));
    }

    #[test]
    fn test_luckperms_loader_build() {
        // Real content from AllayMC/LuckPerms allay/loader/build.gradle
        let content = r#"plugins {
    alias(libs.plugins.shadow)
    alias(libs.plugins.allaygradle)
}

sourceCompatibility = 21
targetCompatibility = 21

allay {
    api = "0.17.0"

    plugin {
        it.entrance = "me.lucko.luckperms.allay.loader.AllayLoaderPlugin"
        it.name = "LuckPerms"
        it.description = "A permissions plugin."
        it.authors = ["Luck"]
        it.version = project.ext.fullVersion
        it.website = "https://luckperms.net"
    }
}

dependencies {
    implementation project(':api')
    implementation project(':common:loader-utils')
}"#;
        let dsl = parse_build_gradle(content).unwrap();
        assert_eq!(dsl.api, Some("0.17.0".to_string()));

        let plugin = dsl.plugin.unwrap();
        assert_eq!(
            plugin.entrance,
            Some("me.lucko.luckperms.allay.loader.AllayLoaderPlugin".to_string())
        );
        assert_eq!(plugin.name, Some("LuckPerms".to_string()));
        assert_eq!(plugin.description, Some("A permissions plugin.".to_string()));
        assert_eq!(plugin.authors, vec!["Luck"]);
        assert_eq!(plugin.website, Some("https://luckperms.net".to_string()));
        // project.ext.fullVersion is a reference, not a literal → None
        assert_eq!(plugin.version, None);
    }
}
