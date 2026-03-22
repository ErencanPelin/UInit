use crate::{
    constants::STEAMWORKS_PACKAGE, fs, new_project::add_package, reporter::Reporter,
    unity_project::UnityProject,
};

pub struct SteamContext {
    pub app_id: u32,
}

pub fn init_steam(
    ctx: &SteamContext,
    unity_project: &UnityProject,
    reporter: &Reporter,
) -> anyhow::Result<()> {
    println!("🚀 Uinit: Initialising steam...");

    // Create steam_appid.txt in the root of the Unity project
    let steam_appid_path = unity_project.root.join("steam_appid.txt");

    reporter.info("Creating steam-appid.txt file.");
    let created = fs::create_file(&steam_appid_path)?;

    if created {
        reporter.success(&format!(
            "Created steam_appid.txt with AppID: {}",
            ctx.app_id
        ));
    } else {
        // it was successful (no errors) but wasn't created (already existed)
        reporter.success(&format!(
            "steam_appid.txt with AppID: {} already exists.",
            ctx.app_id
        ));
    }

    reporter.info("Writing to steam-appid.txt file.");
    fs::write_to_file(&ctx.app_id.to_string(), &steam_appid_path)?;
    reporter.success(&format!(
        "Synced steam_appid.txt with AppID: {}",
        ctx.app_id
    ));

    // add steamworks.net to manifest.json
    add_package(
        unity_project,
        reporter,
        STEAMWORKS_PACKAGE.0,
        STEAMWORKS_PACKAGE.1,
    )?;

    println!("\n✨ Steam initialized successfully.");

    Ok(())
}
