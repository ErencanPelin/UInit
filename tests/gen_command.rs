mod common;
use assert_cmd::Command;
use common::FakeUnityProject;
use tempfile::tempdir;

#[test]
fn gen_creates_expected_folder_structure() {
    let project = FakeUnityProject::new();

    // run init first to create a project structure
    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject"])
        .assert()
        .success();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["gen", "MyNewFeature"])
        .assert()
        .success();

    // validate folders
    for folder in &[
        "Assets/MyGameProject/Scripts/MyNewFeature/Runtime",
        "Assets/MyGameProject/Scripts/MyNewFeature/Editor",
        "Assets/MyGameProject/Scripts/MyNewFeature/Tests",
    ] {
        assert!(project.path().join(folder).is_dir());
    }

    // validate assembly definition files
    for file in &[
        "Assets/MyGameProject/Scripts/MyNewFeature/Runtime/com.DefaultCompany.MyGameProject.MyNewFeature.Runtime.asmdef",
        "Assets/MyGameProject/Scripts/MyNewFeature/Editor/com.DefaultCompany.MyGameProject.MyNewFeature.Editor.asmdef",
        "Assets/MyGameProject/Scripts/MyNewFeature/Tests/com.DefaultCompany.MyGameProject.MyNewFeature.Tests.asmdef",
    ] {
        assert!(project.path().join(file).is_file());
    }
}
