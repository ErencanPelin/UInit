mod common;
use assert_cmd::Command;
use common::FakeUnityProject;

#[test]
fn ci_add_creates_expected_workflow_file() {
    let project = FakeUnityProject::new();

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
        .args(&[
            "ci",
            "add",
            "--host",
            "github",
            "--workflow",
            "editor-tests",
        ])
        .assert()
        .success();

    assert!(
        project
            .path()
            .join(".github/workflows/editor-test.yaml")
            .is_file()
    );
}
