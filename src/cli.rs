use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::alias::AliasType;

#[derive(Parser)]
#[command(author, version, about = "Bootstrap Unity projects faster", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Run in verbose mode
    #[arg(short, long, global = true, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Unity project & uinit.toml
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
    /// Configure complex integrations (Steam, CI, etc.)
    Setup(SetupArgs),
    /// Scaffold a new feature with Runtime, Editor, Tests assemblies
    Gen {
        /// Name of the feature/assembly
        name: String,
        /// Skip creation of the Editor folder
        #[arg(long)]
        no_editor: bool,
        /// Skip creation of the Tests folder
        #[arg(long)]
        no_tests: bool,
    },
    /// Import a remote utility or script via alias
    Add {
        /// The alias defined in your remote/local registry
        alias: String,
    },
    /// Manage project-level aliases
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

#[derive(Args)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub integration: Integration,
}

#[derive(Subcommand)]
pub enum Integration {
    /// Configure Steamworks (requires App ID)
    Steam {
        #[arg(long)]
        app_id: u32,
    },
    /// Configure CI Workflows (GitHub/GitLab)
    Ci {
        #[arg(value_enum)]
        host: CiHost,
        /// The specific workflow template alias
        template: String,
    },
}

#[derive(Subcommand)]
pub enum AliasActions {
    /// List all available aliases
    List {},

    //// Add a new alias mapping to the local config
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

    /// Remove an alias from the local config
    #[command(alias = "rm")]
    Remove {
        /// Alias to be removed
        alias: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CiHost {
    Github,
}
