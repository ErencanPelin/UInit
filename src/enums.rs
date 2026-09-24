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
