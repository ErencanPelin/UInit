use tempfile::TempDir;

pub struct FakeUnityProject {
    pub dir: TempDir, // kept alive for duration of the tests - deleted on drop
}

impl FakeUnityProject {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();

        // create necessary directories
        std::fs::create_dir_all(dir.path().join("Assets")).unwrap();
        std::fs::create_dir_all(dir.path().join("ProjectSettings")).unwrap();
        std::fs::create_dir_all(dir.path().join("Packages")).unwrap();

        // write project settings yaml file
        std::fs::write(
            dir.path().join("ProjectSettings/ProjectSettings.asset"),
            "PlayerSettings:\n  companyName: DefaultCompany\n  productName: MyProject\n",
        )
        .unwrap();

        // write empty manifest.json file
        std::fs::write(
            dir.path().join("Packages/manifest.json"),
            r#"{ "dependencies": {} }"#,
        )
        .unwrap();

        Self { dir }
    }

    pub fn path(&self) -> &std::path::Path {
        self.dir.path()
    }

    pub fn read_file_at_path(&self, relative_path: &str) -> String {
        std::fs::read_to_string(self.dir.path().join(relative_path)).unwrap()
    }

    pub fn path_exists(&self, rel: &str) -> bool {
        self.path().join(rel).exists()
    }
}
