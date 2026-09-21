use assert_cmd::Command;

mod common;

#[test]
fn steam_command_creates_expected_steamworks_file() {
    let project = common::FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject"])
        .assert()
        .success();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["steam", "--app-id", "123456"])
        .assert()
        .success();

    // validate steam_appid.txt file exists
    let steam_appid_path = project.path().join("steam_appid.txt");
    assert!(steam_appid_path.is_file());

    // validate steam_appid.txt file content
    let steam_appid_content = project.read_file_at_path("steam_appid.txt");
    assert_eq!(steam_appid_content.trim(), "123456");
}

#[test]
fn steam_command_does_not_create_steamworks_file_when_using_dry_run() {
    let project = common::FakeUnityProject::new();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["init", "--template", "game", "MyGameProject"])
        .assert()
        .success();

    Command::cargo_bin("uinit")
        .unwrap()
        .current_dir(project.path())
        .args(&["steam", "--app-id", "123456", "--dry-run"])
        .assert()
        .success();

    // validate steam_appid.txt file exists
    let steam_appid_path = project.path().join("steam_appid.txt");
    assert!(!steam_appid_path.is_file());
}
