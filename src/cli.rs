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
        #[arg(short, long, value_enum, default_value_t = ProjectType::Game)]
        template: ProjectType,

        /// Project type
        company: Option<String>,
        /// Email address
        email: Option<String>,
        /// Author name
        author: Option<String>,
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
}

#[derive(Subcommand)]
pub enum SteamActions {
    /// Setup Steam
    Init {
        /// The Steam AppID (use 480 as a temp value)
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ProjectType {
    Game,
    Package,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CiProvider {
    Github,
}
