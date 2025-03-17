use gtk4::ApplicationWindow;
use gtk4::gdk::prelude::SurfaceExt;
use gtk4::prelude::NativeExt;

pub fn hide(window: &ApplicationWindow) {
    let no_cursor = gtk4::gdk::Cursor::from_name("none", None);
    if let Some(surface) = window.surface() {
        surface.set_cursor(no_cursor.as_ref());
    }
}
