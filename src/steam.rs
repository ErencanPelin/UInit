use anyhow::Context;

use crate::{constants::STEAMWORKS_PACKAGE, unity_project::UnityProject};

pub struct SteamContext {
    pub app_id: u32,
}

pub fn init_steam(ctx: &SteamContext, unity_project: &UnityProject) -> anyhow::Result<()> {
    // Create steam_appid.txt in the root of the Unity project
    let steam_appid_path = unity_project.root.join("steam_appid.txt");
    std::fs::write(steam_appid_path, ctx.app_id.to_string())
        .with_context(|| "Failed to write steam_appid.txt")?;

    println!("Created steam_appid.txt with AppID: {}", ctx.app_id);

    // add steamworks.net to manifest.json
    unity_project
        .add_package(STEAMWORKS_PACKAGE.0, STEAMWORKS_PACKAGE.1)
        .with_context(|| "Failed to add Steamworks package to manifest.json")?;

    Ok(())
}
