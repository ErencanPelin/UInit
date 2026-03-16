use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about = "Bootstrap Unity projects faster", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Unity project from a template
    New {
        /// Name of the project directory
        name: String,

        /// Project type
        #[arg(short, long, value_enum)]
        template: ProjectType,

        /// Project type
        #[arg(short, long)]
        company: Option<String>,

        /// Author name
        #[arg(short, long)]
        author: Option<String>,

        /// Email address
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Manage Steamworks integration
    Steam {
        #[command(subcommand)]
        action: SteamActions,
    },
    /// Generate CI/CD configurations
    Ci {
        #[command(subcommand)]
        action: CiActions,
    },
    /// Generate feature / module scaffolding
    Feature {
        #[command(subcommand)]
        action: FeatureActions,
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
pub enum CiActions {
    /// Generate github actions files
    Create {
        #[arg(short, long, default_value = "github")]
        provider: CiProvider,
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
pub enum ProjectType {
    Game,
    Package,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CiProvider {
    Github,
}
