mod configuration;
mod constant;
mod handle_message;

use configuration::Config;
use lipl_display_common::BackgroundThread;
use lipl_gatt_bluer::ListenBluer;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{Layer, layer::SubscriberExt};

slint::include_modules!();

pub(crate) trait ErrorExtension<T> {
    fn err_into(self) -> Result<T, anyhow::Error>;
}

impl<T, E> ErrorExtension<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn err_into(self) -> Result<T, anyhow::Error> {
        self.map_err(|e| e.into())
    }
}

fn setup_logging(config: &Config) -> anyhow::Result<()> {
    tracing_log::LogTracer::builder()
        .with_max_level(config.log_level)
        .init()?;

    let level_filter = LevelFilter::from_level(config.tracing_level);

    let appender = tracing_appender::rolling::Builder::default()
        .filename_prefix(constant::LOG_PREFIX)
        .filename_suffix(constant::LOG_SUFFIX)
        .rotation(Rotation::DAILY)
        .build(&config.log_dir)?;

    let logger = tracing_subscriber::fmt::layer()
        .with_writer(appender)
        .with_filter(level_filter);

    let registry = tracing_subscriber::Registry::default().with(logger);
    tracing::subscriber::set_global_default(registry)?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    let ui = LiplDisplay::new()?;
    ui.set_fontsize(constant::DEFAULT_FONTSIZE);
    ui.set_dark(constant::DEFAULT_DARK);
    let ui_handle = ui.as_weak();

    match configuration::Config::from_file(constant::CONFIG_FILE) {
        Ok(config) => {
            setup_logging(&config)?;
        }
        Err(error) => {
            ui.set_part(
                format!(
                    "Error: {error}\nFile {} missing or parsing error",
                    constant::CONFIG_FILE
                )
                .into(),
            );
        }
    }

    let mut gatt = ListenBluer::new(handle_message::create_handle_message(ui_handle));

    ui.run()?;
    gatt.stop();

    Ok(())
}
