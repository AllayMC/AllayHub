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
        project_version: Option<&str>,
        project_description: Option<&str>,
    ) -> PluginDsl {
        PluginDsl {
            entrance: self.entrance,
            name: self.name,
            version: resolve_template_var(self.version.as_deref(), project_version),
            authors: self.authors,
            description: resolve_description_var(self.description.as_deref(), project_description),
            website: self.website,
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
        json.into_plugin_dsl(None, None)
    }
}

fn resolve_template_var(value: Option<&str>, project_version: Option<&str>) -> Option<String> {
    match value {
        Some(v) if v.contains("${project.version}") => {
            project_version.map(|pv| v.replace("${project.version}", pv))
        }
        Some(v) => Some(v.to_string()),
        None => None,
    }
}

fn resolve_description_var(
    value: Option<&str>,
    project_description: Option<&str>,
) -> Option<String> {
    match value {
        Some(v)
            if v.contains("${description}")
                || v.contains("${project.description}")
                || v.contains("@DESCRIPTION@") =>
        {
            project_description.map(|pd| {
                v.replace("${description}", pd)
                    .replace("${project.description}", pd)
                    .replace("@DESCRIPTION@", pd)
            })
        }
        Some(v) => Some(v.to_string()),
        None => None,
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
        let dsl = json.into_plugin_dsl(Some("1.2.3"), None);
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
        let dsl = json.into_plugin_dsl(None, None);
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
        let dsl = json.into_plugin_dsl(None, Some("A test plugin"));
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
        let dsl = json.into_plugin_dsl(None, Some("Project description"));
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
        let dsl = json.into_plugin_dsl(None, Some("Plugin from @DESCRIPTION@"));
        assert_eq!(dsl.description, Some("Plugin from @DESCRIPTION@".to_string()));
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
        let dsl = json.into_plugin_dsl(None, None);
        assert_eq!(dsl.description, None);
    }
}
