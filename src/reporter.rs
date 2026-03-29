use dialoguer::{Confirm, theme::ColorfulTheme};

pub struct Reporter {
    pub is_verbose: bool,
    pub is_no_prompts: bool,
}

impl Reporter {
    pub fn new(verbose: bool, no_prompts: bool) -> Self {
        Self {
            is_verbose: verbose,
            is_no_prompts: no_prompts,
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

    pub fn prompt(&self, msg: &str) -> bool {
        // default to always accept (respond yes) any prompts that show up
        if self.is_no_prompts {
            return true;
        }

        return Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .default(false)
            .wait_for_newline(true)
            .interact()
            .unwrap_or(false);
    }
}
