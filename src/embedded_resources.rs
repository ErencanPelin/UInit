use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src/resources/vendored/modules/Core/"]
pub struct CoreModuleAssets;

#[derive(Embed)]
#[folder = "src/resources/vendored/modules/Tools/"]
pub struct ToolsModuleAssets;
