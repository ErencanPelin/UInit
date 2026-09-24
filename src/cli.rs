use clap::{Args, Parser, Subcommand};

use crate::enums::{CiHost, ProjectType, WorkflowType};

#[derive(Parser)]
#[command(author, version, about = "Bootstrap Unity projects faster", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Run in verbose mode
    #[arg(short, long, global = true, default_value_t = false)]
    pub verbose: bool,

    /// Always respond 'yes' to any prompts that might appear
    #[arg(short, long, global = true, default_value_t = false)]
    pub no_prompts: bool,

    /// Dry run mode. Show what would be done without making any changes.
    #[arg(short, long, global = true, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Unity project & uinit.toml
    Init {
        /// Name of the project directory
        name: String,

        /// Project type
        #[arg(short, long, value_enum)]
        template: ProjectType,

        /// Project owner company name (used for namespaces and package names). Use your own name if you're not a company
        #[arg(short, long)]
        company: Option<String>,

        /// Email address
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Configure Steamworks (requires App ID)
    Steam {
        #[arg(long)]
        app_id: u32,
    },
    /// Scaffold a new feature with Runtime, Editor, Tests assemblies
    Feature {
        /// Name of the feature/assembly
        name: String,
        /// Skip creation of the Editor folder
        #[arg(long)]
        no_editor: bool,
        /// Skip creation of the Tests folder
        #[arg(long)]
        no_tests: bool,
    },
    /// Add a module or bundle of dependencies
    Add {
        #[command(flatten)]
        target: AddTarget,

        #[arg(long)]
        /// Path starting at project root where the contents of the module will be imported into. E.g. Assets/Scripts/MyModule
        path: Option<String>,
    },
    /// Import a remote utility or script via alias
    Import {
        /// Git URL path of the file or directory you wish to pull into your project
        url: String,

        #[arg(short, long)]
        /// Path starting at project root where the contents of the remote directory will be imported into. E.g. Assets/Scripts/MyModule
        path: String,
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
    /// Set global configuration options for uinit
    Config {
        #[arg(short, long)]
        company: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Configure CI Workflows (GitHub/GitLab)
    Ci {
        #[command(subcommand)]
        action: CiActions,
    },
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct AddTarget {
    #[arg(long)]
    pub bundle: Option<String>,
    #[arg(long)]
    pub module: Option<String>,
}

#[derive(Subcommand)]
pub enum AliasActions {
    /// List all available aliases
    List {},
}

#[derive(Subcommand)]
pub enum CiActions {
    /// List all available workflows
    List {},

    /// Add a new alias mapping to the local config
    Add {
        #[arg(value_enum, long)]
        host: CiHost,
        /// The name of the workflow you want to create. Use --help to see available options.
        #[arg(short, long)]
        workflow: WorkflowType,
    },
}
