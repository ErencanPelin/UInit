use assert_cmd::Command;

mod common;

#[test]
fn import_module_does_not_download_expected_files_when_using_dry_run() {
    let project = common::FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject"])
        .assert()
        .success();

    // download cmd from alias
    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["import", "statemachines", "--dry-run"])
        .assert()
        .success();

    // validate that a new folder was created in Assets/Scripts/Statemachines
    for folder in &["Assets/MyGameProject/Scripts/StateMachines"] {
        assert!(!project.path().join(folder).is_dir());
    }
}

#[test]
fn import_util_does_not_download_expected_files_when_using_dry_run() {
    let project = common::FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject"])
        .assert()
        .success();

    // download cmd from alias
    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["import", "core", "--dry-run"])
        .assert()
        .success();

    // validate that a new folder was created in Assets/Scripts/Statemachines
    for folder in &["Assets/MyGameProject/Scripts/Utils"] {
        assert!(!project.path().join(folder).is_dir());
    }
}
