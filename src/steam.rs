use crate::{constants::STEAMWORKS_PACKAGE, unity_project::UnityProject};

pub struct SteamContext {
    pub app_id: u32,
}

pub fn init_steam(ctx: &SteamContext, unity_project: &UnityProject) {
    // Create steam_appid.txt in the root of the Unity project
    let steam_appid_path = unity_project.root.join("steam_appid.txt");
    std::fs::write(steam_appid_path, ctx.app_id.to_string())
        .expect("Failed to write steam_appid.txt");
    println!("Created steam_appid.txt with AppID: {}", ctx.app_id);

    // add steamworks.net to manifest.json
    let manifest_path = unity_project.packages_dir().join("manifest.json");
    let mut manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path).expect("Failed to read manifest.json"),
    )
    .expect("Failed to parse manifest.json");

    // Add steamworks.net to dependencies if not already present
    let dependencies = manifest
        .get_mut("dependencies")
        .and_then(|d| d.as_object_mut())
        .expect("manifest.json is missing 'dependencies' object");
    if !dependencies.contains_key(STEAMWORKS_PACKAGE.0) {
        dependencies.insert(
            STEAMWORKS_PACKAGE.0.to_string(),
            serde_json::Value::String(STEAMWORKS_PACKAGE.1.to_string()),
        );
        std::fs::write(
            manifest_path,
            serde_json::to_string_pretty(&manifest).expect("Failed to serialize manifest.json"),
        )
        .expect("Failed to write manifest.json");
        println!(
            "Added {} to manifest.json dependencies.",
            STEAMWORKS_PACKAGE.0
        );
    } else {
        println!("com.rlabrecque.steamworks.net is already present in manifest.json dependencies.");
    }
}
