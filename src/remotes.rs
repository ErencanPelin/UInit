use comfy_table::Table;

use crate::{alias::AliasRegistry, reporter::Reporter};

pub fn list_aliases(alias_registry: &AliasRegistry, reporter: &Reporter) -> anyhow::Result<()> {
    reporter.info("Creating table...");
    let mut table = Table::new();

    table.set_header(vec!["Alias", "Type"]);

    let entries = alias_registry.all_aliases();

    for entry in entries {
        table.add_row(vec![&entry.name, &entry.kind.to_string()]);
    }

    println!("{table}");
    Ok(())
}
