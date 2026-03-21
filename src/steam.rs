use crate::{constants::STEAMWORKS_PACKAGE, fs, unity_project::UnityProject};

pub struct SteamContext {
    pub app_id: u32,
}

pub fn init_steam(ctx: &SteamContext, unity_project: &UnityProject) -> anyhow::Result<()> {
    println!("🚀 Uinit: Initialising steam...\n");

    // Create steam_appid.txt in the root of the Unity project
    let steam_appid_path = unity_project.root.join("steam_appid.txt");

    if fs::create_file(&steam_appid_path)? {
        println!("  ✅ Created steam_appid.txt with AppID: {}", ctx.app_id);
    }

    fs::write_to_file(&ctx.app_id.to_string(), &steam_appid_path)?;
    println!("  ✅ Synced steam_appid.txt with AppID: {}", ctx.app_id);

    // add steamworks.net to manifest.json
    unity_project.add_package(STEAMWORKS_PACKAGE.0, STEAMWORKS_PACKAGE.1)?;

    println!("\n✨ Steam initialized successfully.");

    Ok(())
}
