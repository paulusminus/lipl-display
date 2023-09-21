mod constant;
mod handle_message;

use lipl_display_common::BackgroundThread;
use lipl_gatt_bluer::ListenBluer;
use slint::PlatformError;

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(constant::DEFAULT_LOG_LEVEL),
    )
    .init();

    let ui = LiplDisplay::new()?;
    let ui_handle = ui.as_weak();

    ui.set_fontsize(constant::DEFAULT_FONTSIZE);
    ui.set_part(constant::DEFAULT_PART.into());
    ui.set_dark(constant::DEFAULT_DARK);

    let mut gatt = ListenBluer::new(handle_message::create_handle_message(ui_handle));

    ui.run()?;
    gatt.stop();

    Ok(())
}
