// These values are used as defaults for project creation if their values are not provided via CLI arguments.
pub const COMPANY: &str = "ErencanPelin";
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
pub const PACKAGE_JINJA: &str = include_str!("./templates/package.json.jinja2");
pub const LICENSE_JINJA: &str = include_str!("./templates/LICENSE.md.jinja2");
pub const GITIGNORE_TEMPLATE: &str = include_str!("./templates/.gitignore");
pub const CHANGELOG_TEMPLATE: &str = include_str!("./templates/CHANGELOG.md");
pub const ASSEMBLY_DEF_RUNTIME_JINJA: &str =
    include_str!("./templates/assembly_def_runtime.asmdef.jinja2");
pub const ASSEMBLY_DEF_TESTS_JINJA: &str =
    include_str!("./templates/assembly_def_tests.asmdef.jinja2");
pub const ASSEMBLY_DEF_EDITOR_JINJA: &str =
    include_str!("./templates/assembly_def_editor.asmdef.jinja2");

// Package dependencies for manifest.json modifications
pub const STEAMWORKS_PACKAGE: (&str, &str) = (
    "com.rlabrecque.steamworks.net",
    "https://github.com/rlabrecque/Steamworks.NET.git?path=/com.rlabrecque.steamworks.net#2024.8.0",
);
pub const NUGET_MOQ_PACKAGE: (&str, &str) = ("nuget.moq", "2.0.0");

// Aliases for commonly used code that are fetched from external repositories.
// The key is the alias typed by the user, the value is a tuple of (repo_url, path_in_repo).
pub const DEFAULT_ALIASES: &[(&str, &str, &str, &str)] = &[
    (
        "core",
        "https://github.com/ErencanPelin/Unity-Utils",
        "Utils/Core/",
        "util",
    ),
    (
        "predicates",
        "https://github.com/ErencanPelin/Unity-Utils",
        "Utils/Predicates/",
        "util",
    ),
    // TODO: these modules are not setup in the unity-utils repo yet
    // (
    //     "interaction",
    //     "https://github.com/ErencanPelin/Unity-Utils",
    //     "Modules/Interactions/",
    //     "module",
    // ),
    // (
    //     "statemachines",
    //     "https://github.com/ErencanPelin/Unity-Utils",
    //     "Modules/StateMachines/",
    //     "module",
    // ),
    (
        "blender-exporter",
        "https://github.com/ErencanPelin/Unity-Utils",
        "Tools/blender_export.py",
        "tool",
    ),
];
