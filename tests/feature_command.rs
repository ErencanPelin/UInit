mod common;
use assert_cmd::Command;
use common::FakeUnityProject;

#[test]
fn feature_creates_expected_folder_structure() {
    let project = FakeUnityProject::new();

    // run init first to create a project structure
    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&[
            "init",
            "--template",
            "game",
            "MyGameProject",
            "--company",
            "DefaultCompany",
        ])
        .assert()
        .success();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["feature", "MyNewFeature"])
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
        "Assets/MyGameProject/Scripts/MyNewFeature/Runtime/com.defaultcompany.mygameproject.mynewfeature.runtime.asmdef",
        "Assets/MyGameProject/Scripts/MyNewFeature/Editor/com.defaultcompany.mygameproject.mynewfeature.editor.asmdef",
        "Assets/MyGameProject/Scripts/MyNewFeature/Tests/com.defaultcompany.mygameproject.mynewfeature.tests.asmdef",
    ] {
        assert!(project.path().join(file).is_file());
    }
}
