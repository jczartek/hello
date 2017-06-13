extern crate gio;
extern crate gtk;

use gtk::prelude::*;

/// Init Gtk and stuff.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        // run initialization here
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }
    });
}

fn main() {

    init();

    let gapp = gtk::Application::new(Some("com.example.hello"),
                                         gio::APPLICATION_FLAGS_NONE).unwrap();

    gapp.connect_activate(move |gapp| {
        if let Some(win) = gapp.get_active_window() {
            win.present();
        } else {
            let win = gtk::Window::new(gtk::WindowType::Toplevel);
            win.set_default_geometry(600,300);
            gapp.add_window(&win);

            let headerbar = gtk::HeaderBar::new();
            headerbar.set_title(Some("Hello, World!"));
            headerbar.set_show_close_button(true);
            headerbar.show();
            win.set_titlebar(Some(&headerbar));

            let label = gtk::Label::new("<span weight=\"bold\" size=\"larger\">Hello, World!</span>");
            label.set_use_markup(true);
            label.show();
            win.add(&label);

            win.present();
        }
    });

    gapp.run(0, &[]);
}
