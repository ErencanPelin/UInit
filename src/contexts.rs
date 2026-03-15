use crate::args::Cli;
use crate::constants::{AUTHOR, COMPANY, EMAIL};
use chrono::Datelike;

pub struct ProjectContext<'a> {
    pub project_name: &'a str,
    pub company: &'a str,
    pub email: &'a str,
    pub author: &'a str,
    pub year: i32,
}

pub fn get_project_context(cli: &Cli) -> ProjectContext<'_> {
    ProjectContext {
        project_name: cli.project_name.as_str(),
        company: cli.company.as_deref().unwrap_or_else(|| COMPANY),
        email: cli.email.as_deref().unwrap_or_else(|| EMAIL),
        author: cli.author.as_deref().unwrap_or_else(|| AUTHOR),
        year: chrono::Utc::now().year(),
    }
}
