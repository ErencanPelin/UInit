use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    author = "Erencan Pelin",
    version = "0.1.0",
    about = "Create a Unity Assets project structure (game or package)",
    long_about = None
)]
pub struct Cli {
    /// What type of project to create
    #[arg(value_enum)]
    pub project_type: ProjectType,

    /// Name of the project
    #[arg(short, long)]
    pub project_name: String,

    /// Author of the project
    #[arg(short, long)]
    pub author: Option<String>,

    /// Email of the author
    #[arg(short, long)]
    pub email: Option<String>,

    /// Company owner of the project
    #[arg(short, long)]
    pub company: Option<String>,

    /// Just print what would be created (no disk writes)
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(ValueEnum, Clone)]
pub enum ProjectType {
    Game,
    Package,
}
