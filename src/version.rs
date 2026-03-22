use axoupdater::AxoUpdater;
use pollster::FutureExt;

use crate::reporter::Reporter;

pub fn check_for_updates(reporter: &Reporter) -> anyhow::Result<()> {
    let mut updater = AxoUpdater::new_for("uinit");

    if let Ok(loaded) = updater.load_receipt() {
        let is_needed = loaded.is_update_needed().block_on()?;

        if is_needed {
            reporter.info("Update available! Run 'uinit-update'.");
        }
    }

    Ok(())
}
