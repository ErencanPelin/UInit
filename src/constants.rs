use crate::enums::{CiHost, WorkflowType};

// These values are used as defaults for project creation if their values are not provided via CLI arguments.
pub const DEFAULT_COMPANY: &str = "DefaultCompany";
pub const DEFAULT_EMAIL: &str = "";

// Template files contents for file creation and jinja rendering.
pub const PACKAGE_JINJA: &str = include_str!("./resources/templates/package.json.jinja2");
pub const LICENSE_JINJA: &str = include_str!("./resources/templates/LICENSE.jinja2");
pub const GITIGNORE_TEMPLATE: &str = include_str!("./resources/templates/.gitignore");
pub const CHANGELOG_TEMPLATE: &str = include_str!("./resources/templates/CHANGELOG.md");
pub const README_JINJA: &str = include_str!("./resources/templates/README.md.jinja2");
pub const ASSEMBLY_DEF_RUNTIME_JINJA: &str =
    include_str!("./resources/templates/assembly_def_runtime.asmdef.jinja2");
pub const ASSEMBLY_DEF_TESTS_JINJA: &str =
    include_str!("./resources/templates/assembly_def_tests.asmdef.jinja2");
pub const ASSEMBLY_DEF_EDITOR_JINJA: &str =
    include_str!("./resources/templates/assembly_def_editor.asmdef.jinja2");

// Package dependencies for manifest.json modifications
pub const STEAMWORKS_PACKAGE: (&str, &str) = (
    "com.rlabrecque.steamworks.net",
    "https://github.com/rlabrecque/Steamworks.NET.git?path=/com.rlabrecque.steamworks.net#2024.8.0",
);

pub const WORKFLOW_TEMPATES: &[(CiHost, &[(WorkflowType, &str, &str)])] = &[(
    CiHost::Github,
    &[(
        WorkflowType::EditorTests,
        include_str!("./resources/templates/github_workflows/editor_test.yaml"),
        "editor-test.yaml",
    )],
)];
