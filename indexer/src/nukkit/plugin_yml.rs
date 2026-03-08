use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Nukkit plugin.yml 配置结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NukkitPluginYml {
    /// 插件名称
    pub name: String,

    /// 插件版本
    pub version: String,

    /// 主类路径
    pub main: String,

    /// API 版本（支持单个字符串或数组）
    #[serde(default)]
    pub api: ApiVersion,

    /// 单作者（常见于单人项目）
    #[serde(default)]
    pub author: Option<String>,

    /// 作者列表（支持单个字符串或数组）
    #[serde(default)]
    pub authors: Authors,

    /// 插件描述
    #[serde(default)]
    pub description: Option<String>,

    /// 插件网站
    #[serde(default)]
    pub website: Option<String>,

    /// 硬依赖列表
    #[serde(default)]
    pub depend: Vec<String>,

    /// 软依赖列表
    #[serde(default)]
    pub softdepend: Vec<String>,

    /// 加载顺序（STARTUP 或 POSTWORLD）
    #[serde(default)]
    pub load: Option<String>,

    /// 其他未定义的字段
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// API 版本，支持单个字符串或字符串数组
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiVersion {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::Multiple(Vec::new())
    }
}

impl ApiVersion {
    /// 获取所有 API 版本
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            ApiVersion::Single(s) => vec![s.clone()],
            ApiVersion::Multiple(v) => v.clone(),
        }
    }

    /// 获取主要 API 版本
    pub fn primary(&self) -> Option<String> {
        match self {
            ApiVersion::Single(s) => Some(s.clone()),
            ApiVersion::Multiple(v) => v.first().cloned(),
        }
    }
}

/// 作者信息，支持单个字符串或字符串数组
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Authors {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for Authors {
    fn default() -> Self {
        Authors::Multiple(Vec::new())
    }
}

impl Authors {
    /// 获取所有作者
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            Authors::Single(s) => vec![s.clone()],
            Authors::Multiple(v) => v.clone(),
        }
    }
}

impl NukkitPluginYml {
    /// 从 YAML 字符串解析 plugin.yml
    pub fn from_str(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    /// 获取所有依赖（包括硬依赖和软依赖）
    pub fn all_dependencies(&self) -> Vec<String> {
        let mut deps = self.depend.clone();
        deps.extend(self.softdepend.clone());
        deps
    }

    /// 获取所有作者（兼容 author + authors）
    pub fn all_authors(&self) -> Vec<String> {
        let mut merged: Vec<String> = Vec::new();

        if let Some(author) = &self.author {
            let trimmed = author.trim();
            if !trimmed.is_empty() {
                merged.push(trimmed.to_string());
            }
        }

        for name in self.authors.as_vec() {
            let trimmed = name.trim();
            if trimmed.is_empty() || merged.iter().any(|x| x == trimmed) {
                continue;
            }
            merged.push(trimmed.to_string());
        }

        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_plugin_yml() {
        let yaml = r#"
name: ExamplePlugin
version: 1.0.0
main: com.example.ExamplePlugin
api: ["1.0.0"]
authors: ["Author1"]
description: An example plugin
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.name, "ExamplePlugin");
        assert_eq!(plugin.version, "1.0.0");
        assert_eq!(plugin.main, "com.example.ExamplePlugin");
    }

    #[test]
    fn test_parse_single_author() {
        let yaml = r#"
name: TestPlugin
version: 1.0.0
main: com.test.TestPlugin
author: SingleAuthor
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.all_authors(), vec!["SingleAuthor"]);
    }

    #[test]
    fn test_parse_multiple_authors() {
        let yaml = r#"
name: TestPlugin
version: 1.0.0
main: com.test.TestPlugin
authors: ["Author1", "Author2"]
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.all_authors(), vec!["Author1", "Author2"]);
    }

    #[test]
    fn test_parse_author_and_authors_merge() {
        let yaml = r#"
name: TestPlugin
version: 1.0.0
main: com.test.TestPlugin
author: Author1
authors: ["Author1", "Author2"]
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.all_authors(), vec!["Author1", "Author2"]);
    }

    #[test]
    fn test_parse_dependencies() {
        let yaml = r#"
name: TestPlugin
version: 1.0.0
main: com.test.TestPlugin
depend: ["PluginA"]
softdepend: ["PluginB", "PluginC"]
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.depend, vec!["PluginA"]);
        assert_eq!(plugin.softdepend, vec!["PluginB", "PluginC"]);
        assert_eq!(
            plugin.all_dependencies(),
            vec!["PluginA", "PluginB", "PluginC"]
        );
    }

    #[test]
    fn test_api_version_single() {
        let yaml = r#"
name: TestPlugin
version: 1.0.0
main: com.test.TestPlugin
api: "1.0.0"
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.api.primary(), Some("1.0.0".to_string()));
        assert_eq!(plugin.api.as_vec(), vec!["1.0.0"]);
    }

    #[test]
    fn test_api_version_multiple() {
        let yaml = r#"
name: TestPlugin
version: 1.0.0
main: com.test.TestPlugin
api: ["1.0.0", "1.0.1"]
"#;

        let plugin = NukkitPluginYml::from_str(yaml).unwrap();
        assert_eq!(plugin.api.primary(), Some("1.0.0".to_string()));
        assert_eq!(plugin.api.as_vec(), vec!["1.0.0", "1.0.1"]);
    }
}
