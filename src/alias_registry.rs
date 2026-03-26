use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::UinitConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct AliasRegistry {
    pub bundles: HashMap<String, Bundle>,
    pub remotes: HashMap<String, RemoteResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bundle {
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteResource {
    pub url: String,
    pub path: String,
    pub category: String,
}

impl Default for AliasRegistry {
    fn default() -> Self {
        Self {
            bundles: HashMap::new(),
            remotes: HashMap::new(),
        }
    }
}

impl AliasRegistry {
    pub fn load(config: &UinitConfig) -> Self {
        let defaults_str = include_str!("./resources/default_aliases.toml");
        let mut registry: AliasRegistry = toml::from_str(defaults_str)
            .expect("Critical Error: Failed to parse embedded default_aliases.toml");

        registry
            .bundles
            .extend(config.custom_aliases.bundles.clone());

        registry
            .remotes
            .extend(config.custom_aliases.remotes.clone());

        registry
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve_alias(&self, key: &str) -> Option<ResolvedResource> {
        if let Some(bundle) = self.bundles.get(key) {
            return Some(ResolvedResource::Bundle(bundle.dependencies.clone()));
        }
        if let Some(alias) = self.remotes.get(key) {
            return Some(ResolvedResource::Remote(alias.clone()));
        }
        None
    }
}

pub enum ResolvedResource {
    Bundle(Vec<Dependency>),
    Remote(RemoteResource),
}
