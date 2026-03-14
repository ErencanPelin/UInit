pub const COMPANY: &str = "Eren";
pub const AUTHOR: &str = "Erencan Pelin";
pub const EMAIL: &str = "erenp.business@gmail.com";

pub const GAME_PROJECT_TEMPLATE: &'static [&str] = &[
    "/Animations/",
    "/Audio/",
    "/Materials/",
    "/Meshes/",
    "/Prefabs/",
    "/Scenes/",
    "/Scripts/",
    "/Scripts/Core/",
    "/Scripts/Common/",
    "/Shaders/",
    "/Textures/",
    "/README.md",
];

pub const PACKAGE_PROJECT_TEMPLATE: &'static [&str] = &[
    "/Animations/",
    "/Materials/",
    "/Meshes/",
    "/Prefabs/",
    "/Scenes/",
    "/Scripts/",
    "/Scripts/Core/",
    "/Scripts/Common/",
    "/Samples/",
    "/Textures/",
    "/README.md",
    "/LICENSE",
];

pub const PACKAGE_TEMPLATE_JSON: &str = include_str!("./templates/package-template.jinja2.json");
pub const GITIGNORE_CONTENT: &str = include_str!("./templates/.gitignore-template");
pub const LICENSE_CONTENT: &str = include_str!("./templates/LICENSE-template.jinja2");
