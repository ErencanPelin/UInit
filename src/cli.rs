use std::fmt;

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about = "Bootstrap Unity projects faster", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Unity project from a template
    Project {
        #[command(subcommand)]
        action: ProjectActions,
    },
    /// Manage Steamworks integration
    Steam {
        #[command(subcommand)]
        action: SteamActions,
    },
    /// Generate feature / module scaffolding
    Feature {
        #[command(subcommand)]
        action: FeatureActions,
    },
    /// Add utils or features to the project using predefined aliases
    Add {
        /// Alias for the module to be added. Use ``uinit alias list`` to see available options
        alias: String,
    },
    Alias {
        #[command(subcommand)]
        action: AliasActions,
    },
    /// Run diagnostic on your Unity project setup
    Doctor {
        #[arg(short, long, default_value_t = false)]
        fix: bool,
    },
}

#[derive(Subcommand)]
pub enum AliasActions {
    /// List all available aliases
    List {},

    /// Adds a custom alias override to your local uinit.toml config
    Add {
        /// Alias to be used when using ``uinit add``
        alias: String,

        /// Remote repository URL
        #[arg(short, long, value_enum)]
        repo: String,

        /// Path to the module/tool/util from the repository root
        #[arg(short, long, value_enum)]
        path: String,

        /// Remote repository URL
        #[arg(short, long, value_enum)]
        alias_type: AliasType,
    },

    /// Removes a custom alias override from your local uinit.toml config
    Rm {
        /// Alias to be used when using ``uinit add``
        alias: String,
    },
}

#[derive(Subcommand)]
pub enum ProjectActions {
    Init {
        /// Name of the project directory
        name: String,

        /// Project type
        #[arg(short, long, value_enum)]
        template: String,

        /// Project owner company name (used for namespaces and package names). Use your own name if you're not a company
        #[arg(short, long)]
        company: Option<String>,

        /// Email address
        #[arg(short, long)]
        email: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum SteamActions {
    /// Setup Steam
    Init {
        /// The Steam AppID (use 480 as a temp value)
        #[arg(long)]
        app_id: u32,
    },
}

#[derive(Subcommand)]
pub enum FeatureActions {
    /// Scaffold a new game feature with required folders, files and assembly definitions
    Create {
        /// Name of the feature (e.g. inventory, dialogue, etc.)
        name: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CiProvider {
    Github,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, ValueEnum)] // Add ValueEnum here
#[serde(rename_all = "lowercase")]
pub enum AliasType {
    Util,
    Module,
    Tool,
}

// Keep your Display implementation for the 'list' table
impl fmt::Display for AliasType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AliasType::Util => write!(f, "util"),
            AliasType::Module => write!(f, "module"),
            AliasType::Tool => write!(f, "tool"),
        }
    }
}
