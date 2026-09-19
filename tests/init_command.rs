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

    assert!(project.path().join("Assets/MyGameProject/Scripts").is_dir());
    assert!(
        project
            .path()
            .join("Assets/MyGameProject/Animations")
            .is_dir()
    );
    assert!(project.path().join(".gitignore").is_file());
}

#[test]
fn init_package_creates_expected_folder_structure() {
    let project = FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "package", "MyPackageProject"])
        .assert()
        .success();

    assert!(
        project
            .path()
            .join("Assets/MyPackageProject/Scripts")
            .is_dir()
    );
    assert!(
        project
            .path()
            .join("Assets/MyPackageProject/Animations")
            .is_dir()
    );
    assert!(project.path().join(".gitignore").is_file());
}
