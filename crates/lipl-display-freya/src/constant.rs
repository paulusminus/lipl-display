pub const APPLICATION_TITLE: &str = "Lipl Display";
pub const APPLICATION_WIDTH: f64 = 600.0;
pub const APPLICATION_HEIGHT: f64 = 300.0;
pub const WAIT_MESSAGE: &str = "Even geduld a.u.b. ...";
pub const MINIMUM_FONT_SIZE: usize = 4;
pub const FONT_SIZE_INCREMENT: usize = 2;

#[allow(dead_code)]
pub const PATH: &str = "/home/paul/Code/dart/lipl_display/lipl-gatt-input.txt";

#[cfg(feature = "fake")]
pub const INPUT: &[u8] = include_bytes!("/home/paul/Code/dart/lipl_display/lipl-gatt-input.txt");
