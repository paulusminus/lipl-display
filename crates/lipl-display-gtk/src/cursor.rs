use gtk::traits::WidgetExt;
use gtk::ApplicationWindow;

pub fn hide(window: &ApplicationWindow) {
    if let Some(gdk_window) = window.window() {
        let display = gdk_window.display();
        let no_cursor = gtk::gdk::CursorType::BlankCursor;
        let cursor = gtk::gdk::Cursor::for_display(&display, no_cursor);
        if let Some(c) = cursor {
            gdk_window.set_cursor(Some(&c));
        }
    };
}
