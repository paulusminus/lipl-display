mod constant;
mod handle_message;

use lipl_display_common::BackgroundThread;
use lipl_gatt_bluer::ListenBluer;
use slint::PlatformError;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{layer::SubscriberExt, Layer};

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    if let Err(error) = tracing_log::LogTracer::init() {
        eprintln!("Cannot initialize log tracing: {error}");
    }

    let level_filter = LevelFilter::from_level(constant::DEFAULT_LOG_LEVEL);

    match tracing_appender::rolling::Builder::default()
        .filename_prefix(constant::LOG_PREFIX)
        .filename_suffix(constant::LOG_SUFFIX)
        .rotation(Rotation::DAILY)
        .build(constant::LOG_DIR)
    {
        Ok(appender) => {
            let logger = tracing_subscriber::fmt::layer()
                .with_writer(appender)
                .with_filter(level_filter);
            let registry = tracing_subscriber::Registry::default().with(logger);
            if let Err(error) = tracing::subscriber::set_global_default(registry) {
                eprintln!("Cannot set default subscriber: {error}");
            }
        }
        Err(error) => {
            eprintln!("Cannot initialize log file: {error}");
        }
    }

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
