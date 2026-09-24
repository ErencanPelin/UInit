use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bundle {
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddedModule {
    Core,
    Tools,
}

impl EmbeddedModule {
    pub fn folder_name(&self) -> &'static str {
        match self {
            EmbeddedModule::Core => "Core",
            EmbeddedModule::Tools => "Tools",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AliasKind {
    Bundle,
    Module,
}

impl fmt::Display for AliasKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AliasKind::Bundle => write!(f, "Bundle"),
            AliasKind::Module => write!(f, "Module"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AliasEntry {
    pub name: String,
    pub kind: AliasKind,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleDefaults {
    #[serde(default)]
    bundles: HashMap<String, Bundle>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModuleDefaults {
    #[serde(default)]
    embedded: HashMap<String, EmbeddedModule>,
}

pub struct AliasRegistry {
    bundles: HashMap<String, Bundle>,
    embedded: HashMap<String, EmbeddedModule>,
}

impl AliasRegistry {
    pub fn load() -> Self {
        let bundles: BundleDefaults =
            toml::from_str(include_str!("./resources/default_bundle_aliases.toml"))
                .expect("Critical Error: Failed to parse default_bundle_aliases.toml");
        let modules: ModuleDefaults =
            toml::from_str(include_str!("./resources/default_module_aliases.toml"))
                .expect("Critical Error: Failed to parse default_module_aliases.toml");

        AliasRegistry {
            bundles: bundles.bundles,
            embedded: modules.embedded,
        }
    }

    pub fn resolve_bundle(&self, name: &str) -> Option<&Bundle> {
        self.bundles.get(name)
    }

    pub fn resolve_module(&self, name: &str) -> Option<EmbeddedModule> {
        self.embedded.get(name).copied()
    }

    pub fn all_aliases(&self) -> Vec<AliasEntry> {
        let mut entries: Vec<AliasEntry> = self
            .bundles
            .keys()
            .map(|name| AliasEntry {
                name: name.clone(),
                kind: AliasKind::Bundle,
            })
            .chain(self.embedded.keys().map(|name| AliasEntry {
                name: name.clone(),
                kind: AliasKind::Module,
            }))
            .collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }
}
