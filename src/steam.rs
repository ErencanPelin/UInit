use crate::unity_project::UnityProject;

pub struct SteamContext {
    pub app_id: u32,
}

pub fn init_steam(ctx: &SteamContext, unity_project: &UnityProject) {
    // Create steam_appid.txt in the root of the Unity project
    let steam_appid_path = unity_project.root.join("steam_appid.txt");
    std::fs::write(steam_appid_path, ctx.app_id.to_string())
        .expect("Failed to write steam_appid.txt");

    // add steamworks.net to manifest.json
    let manifest_path = unity_project.packages_dir().join("manifest.json");
}
