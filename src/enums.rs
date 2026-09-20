use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CiHost {
    Github,
}

impl fmt::Display for CiHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CiHost::Github => write!(f, "github"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum WorkflowType {
    EditorTests,
}

impl fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorkflowType::EditorTests => write!(f, "editor_tests"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetCategory {
    Util,
    Module,
    Tool,
}

// Keep your Display implementation for the 'list' table
impl fmt::Display for AssetCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetCategory::Util => write!(f, "util"),
            AssetCategory::Module => write!(f, "module"),
            AssetCategory::Tool => write!(f, "tool"),
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Game,
    Package,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectType::Game => write!(f, "game"),
            ProjectType::Package => write!(f, "package"),
        }
    }
}
