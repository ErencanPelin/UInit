mod common;
use assert_cmd::Command;
use common::FakeUnityProject;

#[test]
fn init_game_creates_expected_folder_structure() {
    let project = FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject"])
        .assert()
        .success();

    for folder in &[
        "Assets/MyGameProject/Animations/",
        "Assets/MyGameProject/Audio/",
        "Assets/MyGameProject/Materials/",
        "Assets/MyGameProject/Meshes/",
        "Assets/MyGameProject/Prefabs/",
        "Assets/MyGameProject/Scenes/",
        "Assets/MyGameProject/Scripts/",
        "Assets/MyGameProject/Scripts/Core/",
        "Assets/MyGameProject/Scripts/Common/",
        "Assets/MyGameProject/Shaders/",
        "Assets/MyGameProject/Textures/",
    ] {
        assert!(project.path().join(folder).is_dir());
    }
    assert!(project.path().join(".gitignore").is_file());
}

#[test]
fn init_package_creates_expected_folder_structure() {
    let project = FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&[
            "init",
            "--template",
            "package",
            "MyPackageProject",
            "--company",
            "DefaultCompany",
            "--email",
            "defaultcompany@example.com",
        ])
        .assert()
        .success();

    // validate folders
    for folder in &[
        "Assets/MyPackageProject/Animations/",
        "Assets/MyPackageProject/Materials/",
        "Assets/MyPackageProject/Meshes/",
        "Assets/MyPackageProject/Prefabs/",
        "Assets/MyPackageProject/Scenes/",
        "Assets/MyPackageProject/Scripts/",
        "Assets/MyPackageProject/Scripts/Core/",
        "Assets/MyPackageProject/Scripts/Common/",
        "Assets/MyPackageProject/Samples/",
        "Assets/MyPackageProject/Textures/",
    ] {
        assert!(project.path().join(folder).is_dir());
    }

    for file in &[
        "Assets/MyPackageProject/package.json",
        "Assets/MyPackageProject/README.md",
        "Assets/MyPackageProject/CHANGELOG.md",
        "Assets/MyPackageProject/LICENSE",
    ] {
        assert!(project.path().join(file).is_file());
    }
}

#[test]
fn init_with_dry_run_true_does_not_create_folder_structure() {
    let project = FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject", "--dry-run"])
        .assert()
        .success();

    for folder in &[
        "Assets/MyGameProject/Animations/",
        "Assets/MyGameProject/Audio/",
        "Assets/MyGameProject/Materials/",
        "Assets/MyGameProject/Meshes/",
        "Assets/MyGameProject/Prefabs/",
        "Assets/MyGameProject/Scenes/",
        "Assets/MyGameProject/Scripts/",
        "Assets/MyGameProject/Scripts/Core/",
        "Assets/MyGameProject/Scripts/Common/",
        "Assets/MyGameProject/Shaders/",
        "Assets/MyGameProject/Textures/",
    ] {
        assert!(!project.path().join(folder).is_dir());
    }
    assert!(!project.path().join(".gitignore").is_file());
}
