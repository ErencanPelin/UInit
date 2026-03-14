pub const OWNER: &str = "eren";
pub const OWNER_EMAIL: &str = "erenp.business@gmail.com";

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
    "/LICENSE.md",
];

pub const PACKAGE_TEMPLATE_JSON: &str = include_str!("./templates/package-template.jinja2.json");
