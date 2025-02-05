use cd::select_folder;
use dirs::home_dir;
use gtk4::prelude::*;
use gtk4::{self as gtk, gdk::Display, glib, Button, CssProvider, Image};
use std::cell::RefCell;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use ui::build_ui;
use ui::is_dark_theme_active;

mod cd;
mod ui;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.zoe.com")
        .build();

    application.connect_startup(|app| {
        let provider = CssProvider::new();
        if let Some(home) = home_dir() {
            #[cfg(unix)]
            provider.load_from_path(home.join(".config/zoe/style.css"));
            #[cfg(windows)]
            provider.load_from_path(home.join("AppData\\Local\\zoe\\style.css"));

            gtk::style_context_add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );
            build_ui(app);
        }
    });

    application.run()
}

fn is_hyprland() -> bool {
    // Check for the Hyprland-specific variable
    env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
}

fn buttons_check_sensitive(back_button: Rc<RefCell<Button>>, forward_button: Rc<RefCell<Button>>) {
    let icons;
    if is_dark_theme_active() {
        icons = PathBuf::from("/usr/share/zoe/icons/dark");
    } else {
        icons = PathBuf::from("/usr/share/zoe/icons/light");
    }

    let back_button_icon_innactive = Image::from_file(icons.join("back-button-innactive.png"));
    let forward_button_icon_innactive =
        Image::from_file(icons.join("forward-button-innactive.png"));
    let back_button_icon_active = Image::from_file(icons.join("back-button-active.png"));
    let forward_button_icon_active = Image::from_file(icons.join("forward-button-active.png"));

    if back_button.borrow().is_sensitive() {
        back_button
            .borrow()
            .set_child(Some(&back_button_icon_active.clone()));
    } else {
        back_button
            .borrow()
            .set_child(Some(&back_button_icon_innactive.clone()));
    }

    if forward_button.borrow().is_sensitive() {
        forward_button
            .borrow()
            .set_child(Some(&forward_button_icon_active.clone()));
    } else {
        forward_button
            .borrow()
            .set_child(Some(&forward_button_icon_innactive.clone()));
    }
}
