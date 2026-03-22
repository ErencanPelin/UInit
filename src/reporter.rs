pub struct Reporter {
    pub is_verbose: bool,
}

impl Reporter {
    pub fn new(verbose: bool) -> Self {
        Self {
            is_verbose: verbose,
        }
    }

    pub fn success(&self, msg: &str) {
        println!("  ✅ {}", msg);
    }

    pub fn info(&self, msg: &str) {
        if self.is_verbose {
            println!("      [info] {}", msg);
        }
    }
}
