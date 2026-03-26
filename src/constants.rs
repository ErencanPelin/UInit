// These values are used as defaults for project creation if their values are not provided via CLI arguments.
pub const DEFAULT_COMPANY: &str = "DefaultCompany";
pub const DEFAULT_EMAIL: &str = "";

/*
 * Template paths for project creation. The {} in the paths will be replaced with the project name.
 * End the line with a / to create a directory, otherwise a file will be created.
*/
pub const PROJECT_TEMPLATES: &[(&str, &[&str], &[(&str, &str)])] = &[
    (
        "game", // alias to create the template
        &[
            // folder structure
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
            "Assets/../.gitignore",
        ],
        &[("com.unity.nuget.newtonsoft-json", "3.2.2")], // dependencies
    ),
    (
        "package",
        &[
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
            "Assets/../.gitignore",
        ],
        &[("com.unity.nuget.newtonsoft-json", "3.2.2")], // dependencies
    ),
];

// Template files contents for file creation and jinja rendering.
pub const PACKAGE_JINJA: &str = include_str!("./templates/package.json.jinja2");
pub const LICENSE_JINJA: &str = include_str!("./templates/LICENSE.jinja2");
pub const GITIGNORE_TEMPLATE: &str = include_str!("./templates/.gitignore");
pub const CHANGELOG_TEMPLATE: &str = include_str!("./templates/CHANGELOG.md");
pub const README_JINJA: &str = include_str!("./templates/README.md.jinja2");
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
