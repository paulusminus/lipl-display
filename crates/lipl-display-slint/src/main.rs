mod constant;
mod handle_message;

use lipl_display_common::BackgroundThread;
use lipl_gatt_bluer::ListenBluer;
use slint::PlatformError;

slint::include_modules!();

// const AMSTERDAMSE_GRACHTEN_TEXT: &str = "Aan de amsterdamse grachten heb ik heel mijn hart voor altijd verpland\nAmsterdam kent mij gedachten als de mooiste stad in ons land\nAl die amsterdamse mensen al die lichtjes 's avonds laat op het plein\nNiemand kan zich beter wensen dan een amsterdammer te zijn";
// const AMSTERDAMSE_GRACHTEN_STATUS: &str = "Aan de amsterdamse grachten (1/4)";


fn main() -> Result<(), PlatformError> {

    let ui = LiplDisplay::new()?;
    let ui_handle = ui.as_weak();
  
    ui.set_whatever(30);
    ui.set_part(constant::JUST_A_MOMENT.into());
    ui.set_dark(true);

    let mut gatt = ListenBluer::new(handle_message::create_handle_message(ui_handle));

    ui.run()?;
    gatt.stop();

    Ok(())
}