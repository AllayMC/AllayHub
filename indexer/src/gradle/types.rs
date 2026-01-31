use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub enum VersionRef {
    #[default]
    None,
    Literal(String),
    VersionCatalog(String),
    Variable(String),
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AllayDsl {
    pub api: Option<String>,
    pub api_only: Option<bool>,
    pub server: Option<String>,
    pub plugin: Option<PluginDsl>,
    #[serde(default)]
    pub has_allay_dependency: bool,
    #[serde(skip)]
    pub api_version_ref: VersionRef,
    #[serde(skip)]
    pub server_version_ref: VersionRef,
    #[serde(skip)]
    pub project_name: Option<String>,
    #[serde(skip)]
    pub project_version: Option<String>,
    #[serde(skip)]
    pub project_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PluginDsl {
    pub entrance: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub api_version: Option<String>,
    pub dependencies: Vec<GradleDependency>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GradleDependency {
    pub name: String,
    pub version: Option<String>,
    pub optional: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PluginJson {
    pub entrance: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub website: Option<String>,
    pub api_version: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<PluginJsonDependency>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PluginJsonDependency {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub optional: bool,
}

impl PluginJson {
    pub fn into_plugin_dsl(
        self,
        project_name: Option<&str>,
        project_version: Option<&str>,
        project_description: Option<&str>,
    ) -> PluginDsl {
        let ctx = TemplateContext {
            name: project_name,
            version: project_version,
            description: project_description,
        };
        PluginDsl {
            entrance: self.entrance,
            name: resolve_template(self.name.as_deref(), &ctx),
            version: resolve_template(self.version.as_deref(), &ctx),
            authors: self.authors,
            description: resolve_template(self.description.as_deref(), &ctx),
            website: resolve_template(self.website.as_deref(), &ctx),
            api_version: self.api_version,
            dependencies: self
                .dependencies
                .into_iter()
                .map(|d| GradleDependency {
                    name: d.name,
                    version: d.version,
                    optional: d.optional,
                })
                .collect(),
        }
    }
}

impl From<PluginJson> for PluginDsl {
    fn from(json: PluginJson) -> Self {
        json.into_plugin_dsl(None, None, None)
    }
}

struct TemplateContext<'a> {
    name: Option<&'a str>,
    version: Option<&'a str>,
    description: Option<&'a str>,
}

/// Resolve Gradle `processResources` template variables in a string.
/// Known patterns: `${name}`, `${project.name}`, `${version}`, `${project.version}`,
/// `${description}`, `${project.description}`, `@DESCRIPTION@`.
/// Returns None if any `${...}` placeholder remains unresolved.
fn resolve_template(value: Option<&str>, ctx: &TemplateContext) -> Option<String> {
    let v = value?;
    if !v.contains("${") && !v.contains("@DESCRIPTION@") {
        return Some(v.to_string());
    }

    let mut result = v.to_string();

    // Resolve name
    if let Some(name) = ctx.name {
        result = result
            .replace("${name}", name)
            .replace("${project.name}", name);
    }

    // Resolve version
    if let Some(version) = ctx.version {
        result = result
            .replace("${version}", version)
            .replace("${project.version}", version);
    }

    // Resolve description
    if let Some(description) = ctx.description {
        result = result
            .replace("${description}", description)
            .replace("${project.description}", description)
            .replace("@DESCRIPTION@", description);
    }

    // If any template variable remains unresolved, return None
    if result.contains("${") || result.contains("@DESCRIPTION@") {
        None
    } else {
        Some(result)
    }
}

pub fn parse_plugin_json(content: &str) -> Option<PluginJson> {
    serde_json::from_str(content).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_json_parsing() {
        let content = r#"{
            "entrance": "cn.huohuas001.huhobot.allay.HuHoBotAllay",
            "name": "HuHoBot",
            "authors": ["HuoHuas001"],
            "version": "0.1.1",
            "api_version": ">=0.14.0"
        }"#;
        let json = parse_plugin_json(content).unwrap();
        assert_eq!(
            json.entrance,
            Some("cn.huohuas001.huhobot.allay.HuHoBotAllay".to_string())
        );
        assert_eq!(json.name, Some("HuHoBot".to_string()));
        assert_eq!(json.authors, vec!["HuoHuas001"]);
        assert_eq!(json.version, Some("0.1.1".to_string()));
        assert_eq!(json.api_version, Some(">=0.14.0".to_string()));
    }

    #[test]
    fn test_plugin_json_to_plugin_dsl() {
        let json = PluginJson {
            entrance: Some("Test".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("1.0.0".to_string()),
            authors: vec!["Author1".to_string()],
            api_version: Some(">=0.14.0".to_string()),
            ..Default::default()
        };
        let dsl: PluginDsl = json.into();
        assert_eq!(dsl.entrance, Some("Test".to_string()));
        assert_eq!(dsl.name, Some("TestPlugin".to_string()));
        assert_eq!(dsl.api_version, Some(">=0.14.0".to_string()));
    }

    #[test]
    fn test_plugin_json_with_project_version_template() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("${project.version}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, Some("1.2.3"), None);
        assert_eq!(dsl.version, Some("1.2.3".to_string()));
    }

    #[test]
    fn test_plugin_json_with_project_version_template_no_version() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("${project.version}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, None);
        assert_eq!(dsl.version, None);
    }

    #[test]
    fn test_plugin_json_with_description_template() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("1.0.0".to_string()),
            description: Some("${description}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, Some("A test plugin"));
        assert_eq!(dsl.description, Some("A test plugin".to_string()));
    }

    #[test]
    fn test_plugin_json_with_project_description_template() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("1.0.0".to_string()),
            description: Some("${project.description}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, Some("Project description"));
        assert_eq!(dsl.description, Some("Project description".to_string()));
    }

    #[test]
    fn test_plugin_json_with_at_description_template() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("1.0.0".to_string()),
            description: Some("@DESCRIPTION@".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, Some("An awesome plugin"));
        assert_eq!(dsl.description, Some("An awesome plugin".to_string()));
    }

    #[test]
    fn test_plugin_json_with_at_description_template_no_project_desc() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("1.0.0".to_string()),
            description: Some("@DESCRIPTION@".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, None);
        assert_eq!(dsl.description, None);
    }

    #[test]
    fn test_plugin_json_with_description_template_no_project_desc() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("TestPlugin".to_string()),
            version: Some("1.0.0".to_string()),
            description: Some("${description}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, None);
        assert_eq!(dsl.description, None);
    }

    #[test]
    fn test_plugin_json_with_name_template() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("${name}".to_string()),
            version: Some("${version}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(Some("Pronouns"), Some("2.0.0"), None);
        assert_eq!(dsl.name, Some("Pronouns".to_string()));
        assert_eq!(dsl.version, Some("2.0.0".to_string()));
    }

    #[test]
    fn test_plugin_json_with_name_template_no_project_name() {
        let json = PluginJson {
            entrance: Some("com.example.Plugin".to_string()),
            name: Some("${name}".to_string()),
            authors: vec!["Author".to_string()],
            ..Default::default()
        };
        let dsl = json.into_plugin_dsl(None, None, None);
        assert_eq!(dsl.name, None);
    }
}
