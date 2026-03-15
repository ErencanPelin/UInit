// use crate::cli::{Command, ProjectType};
// use crate::constants::{AUTHOR, COMPANY, EMAIL};
// use chrono::Datelike;

// pub struct ProjectContext<'a> {
//     pub project_type: ProjectType,
//     pub project_name: &'a str,
//     pub company: &'a str,
//     pub email: &'a str,
//     pub author: &'a str,
//     pub year: i32,
// }

// pub struct SteamContext<'a> {
//     pub app_id: &'a str,
// }

// pub fn get_project_context(cmd: &Command) -> Result<ProjectContext<'_>, &'static str> {
//     if let Command::Project {
//         project_type,
//         project_name,
//         company,
//         email,
//         author,
//     } = cmd
//     {
//         Ok()
//     } else {
//         Err("Not a Project command")
//     }
// }

// pub fn get_steam_context(cmd: &Command) -> Result<SteamContext<'_>, &'static str> {
//     if let Command::Steam { app_id } = cmd {
//         Ok(SteamContext {
//             app_id: app_id.as_str(),
//         })
//     } else {
//         Err("Not a Steam command")
//     }
// }
