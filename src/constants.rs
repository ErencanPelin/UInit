// These values are used as defaults for project creation if their values are not provided via CLI arguments.
pub const COMPANY: &str = "Eren";
pub const AUTHOR: &str = "Erencan Pelin";
pub const EMAIL: &str = "erenp.business@gmail.com";

/*
 * Template paths for project creation. The {} in the paths will be replaced with the project name.
 * End the line with a / to create a directory, otherwise a file will be created.
*/
pub const GAME_PROJECT_TEMPLATE: &[&str] = &[
    "Assets/{}/Animations/",
    "Assets/{}/Audio/",
    "Assets/{}/Materials/",
    "Assets/{}/Meshes/",
    "Assets/{}/Prefabs/",
    "Assets/{}/Scenes/",
    "Assets/{}/Scripts/",
    "Assets/{}/Scripts/Core/",
    "Assets/{}/Scripts/Common/",
    "Assets/{}/Shaders/",
    "Assets/{}/Textures/",
    "README.md",
];

pub const PACKAGE_PROJECT_TEMPLATE: &[&str] = &[
    "Assets/{}/Animations/",
    "Assets/{}/Materials/",
    "Assets/{}/Meshes/",
    "Assets/{}/Prefabs/",
    "Assets/{}/Scenes/",
    "Assets/{}/Scripts/",
    "Assets/{}/Scripts/Core/",
    "Assets/{}/Scripts/Common/",
    "Assets/{}/Samples/",
    "Assets/{}/Textures/",
    "Assets/{}/README.md",
    "Assets/{}/CHANGELOG.md",
    "Assets/{}/LICENSE",
    "Assets/{}/package.json",
];

// Template files contents for file creation and jinja rendering.
pub const PACKAGE_JINJA: &str = include_str!("./templates/package_template.json.jinja2");
pub const LICENSE_JINJA: &str = include_str!("./templates/LICENSE_template.md.jinja2");
pub const GITIGNORE_TEMPLATE: &str = include_str!("./templates/.gitignore_template");
pub const CHANGELOG_TEMPLATE: &str = include_str!("./templates/CHANGELOG_template.md");

// Package dependencies for manifest.json modifications
pub const STEAMWORKS_PACKAGE: (&str, &str) = (
    "com.rlabrecque.steamworks.net",
    "https://github.com/rlabrecque/Steamworks.NET.git?path=/com.rlabrecque.steamworks.net#2024.8.0",
);
