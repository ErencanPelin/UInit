use axoupdater::AxoUpdater;

use crate::reporter::Reporter;

pub fn check_for_updates(reporter: &Reporter) -> anyhow::Result<()> {
    let mut updater = AxoUpdater::new_for("uinit");

    if let Ok(loaded) = updater.load_receipt() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let is_needed = runtime.block_on(loaded.is_update_needed())?;

        if is_needed {
            reporter.info("Update available! Run 'uinit-update'.");
        }
    }

    Ok(())
}
