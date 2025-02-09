use dirs::home_dir;
use gdk4::Rectangle;
use gtk4::{self as gtk, Box, Button, EventControllerMotion, HeaderBar, Image, Label, Popover};
use gtk4::{
    gdk::Display, prelude::*, ApplicationWindow, Entry, FlowBox, GestureClick, Orientation,
    ScrolledWindow, Settings,
};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::buttons_check_sensitive;
use crate::cd::delete;
use crate::is_hyprland;
use crate::select_folder;
use crate::ui_b::empty_space_pop;

pub fn build_ui(app: &gtk::Application) {
    if let Some(home) = home_dir() {
        let history = Rc::new(RefCell::new(Vec::new()));
        let current_pos = Rc::new(RefCell::new(0usize));
        let copy_memory = Rc::new(RefCell::new(PathBuf::new()));

        let window = gtk::ApplicationWindow::builder()
            .default_width(600)
            .default_height(600)
            .application(app)
            .title("Zoe")
            .build();

        window.add_css_class("main-window");

        let header_bar;
        if is_hyprland() {
            header_bar = HeaderBar::builder().show_title_buttons(false).build();
        } else {
            header_bar = HeaderBar::builder().show_title_buttons(true).build();
        }

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
        let back_button_icon_hover = Image::from_file(icons.join("back-button-hover.png"));
        let forward_button_icon_hover = Image::from_file(icons.join("forward-button-hover.png"));
        // let back_button = Button::builder().child(&back_button_icon_innactive).build();
        let back_button = Rc::new(RefCell::new(
            Button::builder().child(&back_button_icon_innactive).build(),
        ));
        let forward_button = Rc::new(RefCell::new(
            Button::builder()
                .child(&forward_button_icon_innactive)
                .build(),
        ));
        back_button.borrow().add_css_class("back_button");
        forward_button.borrow().add_css_class("forward_button");
        forward_button.borrow().add_css_class("but");
        back_button.borrow().add_css_class("but");

        back_button.borrow().set_sensitive(false);
        forward_button.borrow().set_sensitive(false);

        // Add buttons to the HeaderBar
        header_bar.pack_start(&back_button.borrow().clone());
        header_bar.pack_start(&forward_button.borrow().clone());

        let flow_box = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .max_children_per_line(1)
            .min_children_per_line(1)
            .selection_mode(gtk::SelectionMode::None)
            .build();

        flow_box.add_css_class("flowbox");
        let flow_box_clone = flow_box.clone();

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(1)
            .hexpand(true)
            .vexpand(true)
            .child(&flow_box)
            .build();

        let window_clone = window.clone();
        let scrolled_window_clone = scrolled_window.clone();
        let copy_memory_clone = copy_memory.clone();
        let history_clone = history.clone();
        let current_pos_clone = current_pos.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();

        let gesture_click = GestureClick::new();
        gesture_click.set_button(3);

        gesture_click.connect_pressed(move |_gesture, _n_press, x, y| {
            // Check if the click happened on a child widget
            let child_widget = flow_box_clone.child_at_pos(x as i32, y as i32);

            if child_widget.is_none() && (881 > x as i32) {
                empty_space_pop(
                    x,
                    y,
                    scrolled_window_clone.clone(),
                    copy_memory.clone(),
                    history_clone.clone(),
                    current_pos_clone.clone(),
                    flow_box_clone.clone(),
                    back_button_clone.clone(),
                    forward_button_clone.clone(),
                    window_clone.clone(),
                );
            }
        });

        let scrolled_window_clone = scrolled_window.clone();
        let window_clone = window.clone();

        gesture_click.set_propagation_phase(gtk::PropagationPhase::Capture);
        scrolled_window.add_controller(gesture_click);

        // Connect the click event
        let flow_box_clone = flow_box.clone();

        select_folder(
            home,
            flow_box_clone,
            history.clone(),
            current_pos.clone(),
            back_button.clone(),
            forward_button.clone(),
            true,
            window_clone.clone(),
            scrolled_window.clone(),
            copy_memory_clone.clone(),
        );

        window_clone.set_titlebar(Some(&header_bar));
        window_clone.set_child(Some(&scrolled_window));
        app.connect_activate(move |_| {
            window_clone.present();
        });

        let back_motion_controller = EventControllerMotion::new();
        {
            let back_button_clone = back_button.clone();
            let back_hover_icon = back_button_icon_hover.clone();
            back_motion_controller.connect_enter(move |_, _, _| {
                back_button_clone.borrow().set_child(Some(&back_hover_icon));
            });
        }
        {
            let back_button_clone = back_button.clone();
            let back_active_icon = back_button_icon_active.clone();
            back_motion_controller.connect_leave(move |_| {
                back_button_clone
                    .borrow()
                    .set_child(Some(&back_active_icon));
            });
        }

        // Attach the motion controller to the button
        back_button.borrow().add_controller(back_motion_controller);

        let forward_motion_controller = EventControllerMotion::new();
        {
            let forward_button_clone = forward_button.clone();
            let forward_hover_icon = forward_button_icon_hover.clone();
            forward_motion_controller.connect_enter(move |_, _, _| {
                forward_button_clone
                    .borrow()
                    .set_child(Some(&forward_hover_icon));
            });
        }
        {
            let forward_button_clone = forward_button.clone();
            let forward_active_icon = forward_button_icon_active.clone();
            forward_motion_controller.connect_leave(move |_| {
                forward_button_clone
                    .borrow()
                    .set_child(Some(&forward_active_icon));
            });
        }

        // Attach the motion controller to the button
        forward_button
            .borrow()
            .add_controller(forward_motion_controller);

        {
            back_button.borrow().connect_clicked({
                let copy_memory_clone = copy_memory_clone.clone();
                let history_clone = history.clone();
                let current_pos_clone = current_pos.clone();
                let flow_box_clone = flow_box.clone();
                let back_button_clone = back_button.clone();
                let forward_button_clone = forward_button.clone();
                let window_clone = window.clone();

                move |_| {
                    // Use a shorter scope for borrowing
                    let path_to_navigate = {
                        let mut current_pos = current_pos_clone.borrow_mut();

                        if *current_pos > 0 {
                            *current_pos -= 1;
                            let history = history_clone.borrow();
                            Some(history[*current_pos].clone()) // Return only the PathBuf
                        } else {
                            None
                        }
                    };

                    // If there's a valid path to navigate, call `select_folder`
                    if let Some(path) = path_to_navigate {
                        if path.is_dir() {
                            while let Some(child) = flow_box_clone.first_child() {
                                flow_box_clone.remove(&child); // Use reference to child
                            }
                            select_folder(
                                path,
                                flow_box_clone.clone(),
                                history_clone.clone(),
                                current_pos_clone.clone(),
                                back_button_clone.clone(),
                                forward_button_clone.clone(),
                                false, // Do not modify history
                                window_clone.clone(),
                                scrolled_window.clone(),
                                copy_memory_clone.clone(),
                            );
                        }
                    }

                    // Update button sensitivity (short borrow scope here)
                    let pos = *current_pos_clone.borrow();
                    let hist_len = history_clone.borrow().len();
                    back_button_clone.borrow().set_sensitive(pos > 0);
                    forward_button_clone
                        .borrow()
                        .set_sensitive(pos + 1 < hist_len);

                    buttons_check_sensitive(
                        back_button_clone.clone(),
                        forward_button_clone.clone(),
                    );
                }
            });
        }

        {
            forward_button.borrow().connect_clicked({
                let history_clone = history.clone();
                let current_pos_clone = current_pos.clone();
                let flow_box_clone = flow_box.clone();
                let back_button_clone = back_button.clone();
                let forward_button_clone = forward_button.clone();
                let copy_memory_clone = copy_memory_clone.clone();
                let window_clone = window.clone();

                move |_| {
                    let path_to_navigate = {
                        let mut current_pos = current_pos_clone.borrow_mut();
                        let history_len = history_clone.borrow().len();

                        if *current_pos + 1 < history_len {
                            *current_pos += 1;
                            let history = history_clone.borrow();
                            Some(history[*current_pos].clone()) // Return only PathBuf
                        } else {
                            None
                        }
                    };

                    if let Some(path) = path_to_navigate {
                        if path.is_dir() {
                            while let Some(child) = flow_box_clone.first_child() {
                                flow_box_clone.remove(&child); // Use reference to child
                            }
                            select_folder(
                                path,
                                flow_box_clone.clone(),
                                history_clone.clone(),
                                current_pos_clone.clone(),
                                back_button_clone.clone(),
                                forward_button_clone.clone(),
                                false, // Do not modify history
                                window_clone.clone(),
                                scrolled_window_clone.clone(),
                                copy_memory_clone.clone(),
                            );
                        }
                    }

                    // Update button sensitivity (short borrow scope here)
                    let pos = *current_pos_clone.borrow();
                    let hist_len = history_clone.borrow().len();
                    back_button_clone.borrow().set_sensitive(pos > 0);
                    forward_button_clone
                        .borrow()
                        .set_sensitive(pos + 1 < hist_len);
                    buttons_check_sensitive(
                        back_button_clone.clone(),
                        forward_button_clone.clone(),
                    );
                }
            });
        }
    }
}

pub fn pop_up(
    button: Button,
    x: f64,
    y: f64,
    new_item: String,
    item: String,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
) {
    let popup_options = vec!["Open", "Cut", "Copy", "Move", "Rename", "Delete"];
    let popup = Popover::builder().has_arrow(false).build();
    let vbox = Box::new(Orientation::Vertical, 5);

    for option in popup_options.iter() {
        let label = Label::new(Some(option));
        label.set_xalign(0.01);
        label.add_css_class("files_color");
        label.add_css_class("p_but_label");
        let button_pop = Button::builder().child(&label).build();
        // Connect the click event
        let window_clone = window.clone();
        let popup_clone = popup.clone();
        let option_clone = option.to_string().clone();
        let new_item = new_item.clone();
        let item_clone = item.clone();

        let flow_box_clone = flow_box.clone();
        let history_clone = history.clone();
        let current_pos_clone = current_pos.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();
        let scr_clone = scr.clone();
        let button_clone = button.clone();
        let copy_memory_clone = copy_memory.clone();

        button_pop.add_css_class("flat");

        button_pop.connect_clicked(move |_| {
            popup_clone.popdown();
            let item_path = PathBuf::from(item_clone.clone());
            if option_clone == "Delete" {
                // gtk::glib::MainContext::default()
                //     .spawn_local(dialog_delete(window_clone.clone(), item.clone()));
                dialog_delete(
                    new_item.clone(),
                    item_clone.clone(),
                    flow_box_clone.clone(),
                    history_clone.clone(),
                    current_pos_clone.clone(),
                    back_button_clone.clone(),
                    forward_button_clone.clone(),
                    window_clone.clone(),
                    scr_clone.clone(),
                    copy_memory_clone.clone(),
                );
            } else if option_clone == "Rename" {
                dialog_rename(
                    new_item.clone(),
                    item_clone.clone(),
                    flow_box_clone.clone(),
                    history_clone.clone(),
                    current_pos_clone.clone(),
                    back_button_clone.clone(),
                    forward_button_clone.clone(),
                    window_clone.clone(),
                    scr_clone.clone(),
                    button_clone.clone(),
                    copy_memory_clone.clone(),
                );
            } else if option_clone == "Open" {
                if item_path.is_dir() && item_path.exists() {
                    while let Some(child) = flow_box_clone.first_child() {
                        flow_box_clone.remove(&child);
                    }
                    select_folder(
                        item_path,
                        flow_box_clone.clone(),
                        history_clone.clone(),
                        current_pos_clone.clone(),
                        back_button_clone.clone(),
                        forward_button_clone.clone(),
                        true,
                        window_clone.clone(),
                        scr_clone.clone(),
                        copy_memory_clone.clone(),
                    );
                }
            } else if option_clone == "Copy" {
                if item_path.exists() {
                    let mut copy_memory_mut = copy_memory_clone.borrow_mut();
                    *copy_memory_mut = item_path;
                }
            }
        });

        vbox.append(&button_pop);
    }
    popup.set_child(Some(&vbox));

    let wx = x as i32 + 35;
    let wy = y as i32 - 10;

    let rect = Rectangle::new(wx, wy, 1, 1);
    // println!("Cur - {} : {}", bx, y);
    popup.set_pointing_to(Some(&rect));

    popup.set_parent(&button);
    popup.popup();
}

fn dialog_rename(
    new_item: String,
    item: String,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
    scr: ScrolledWindow,
    button: Button,
    copy_memory: Rc<RefCell<PathBuf>>,
) {
    let item_path = PathBuf::from(&item);
    let popup = Popover::builder().has_arrow(true).build();

    let mes;
    if item_path.is_dir() {
        mes = "Rename Folder";
    } else {
        mes = "Rename File";
    }

    let title = Label::new(Some(&mes));
    title.add_css_class("del_dialog_title");

    let entry = Entry::new();
    entry.set_text(&new_item);
    entry.add_css_class("suggested-action");
    entry.set_width_chars(30);

    let but_ren = Button::builder().label("Rename").build();
    but_ren.add_css_class("suggested-action");

    let hbox = Box::new(Orientation::Horizontal, 0);
    hbox.append(&but_ren);
    hbox.set_halign(gtk4::Align::End);

    let vbox = Box::new(Orientation::Vertical, 20);
    vbox.append(&title);
    vbox.append(&entry);
    vbox.append(&hbox);
    vbox.set_valign(gtk4::Align::Center);
    vbox.add_css_class("del_dialog_vbox");
    vbox.add_css_class("ren_dialog_vbox");

    let x = button.width() / 2;
    let y = button.height();
    let rect = Rectangle::new(x, y, 1, 1);
    popup.set_child(Some(&vbox));
    popup.set_pointing_to(Some(&rect));
    popup.set_parent(&button);

    let popup_clone = popup.clone();
    but_ren.connect_clicked(move |_| {
        popup_clone.popdown();
        let item_path = item_path.clone();
        let flow_box = flow_box.clone();
        let history = history.clone();
        let current_pos = current_pos.clone();
        let back_button = back_button.clone();
        let forward_button = forward_button.clone();
        let window = window.clone();
        let scr = scr.clone();
        let current_dir = {
            let current_pos_clone = current_pos.borrow_mut();

            let history_clone = history.borrow();
            Some(history_clone[*current_pos_clone].clone()) // Return only the PathBuf
        };
        let copy_memory_clone = copy_memory.clone();
        if let Some(path) = current_dir {
            if path.exists() {
                fs::rename(item_path, path.clone().join(entry.text()))
                    .expect("Failed to rename file.");
                while let Some(child) = flow_box.first_child() {
                    flow_box.remove(&child);
                }
                select_folder(
                    path,
                    flow_box,
                    history,
                    current_pos,
                    back_button,
                    forward_button,
                    false,
                    window,
                    scr,
                    copy_memory_clone.clone(),
                );
            }
        }
    });

    popup.popup();
}

fn dialog_delete(
    new_item: String,
    item: String,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
) {
    let popup = Popover::builder().has_arrow(false).build();
    let mes = format!("Permanently Delete \"{}\"?", new_item);
    let title = Label::new(Some(&mes));
    title.add_css_class("del_dialog_title");
    let message = Label::new(Some("Permanently deleted items can't be restored"));
    let but_cancel = Button::builder().label("Cancel").build();
    let but_del = Button::builder().label("Delete").build();
    but_del.add_css_class("destructive-action");
    but_cancel.add_css_class("accent");

    let hbox = Box::new(Orientation::Horizontal, 20);
    hbox.append(&but_cancel);
    hbox.append(&but_del);
    hbox.set_halign(gtk4::Align::Center);

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.append(&title);
    vbox.append(&message);
    vbox.append(&hbox);
    vbox.set_valign(gtk4::Align::Center);
    vbox.add_css_class("del_dialog_vbox");

    let x = window.width() / 2;
    let y = (window.height() / 2) - 50;
    let rect = Rectangle::new(x, y, 1, 1);
    popup.set_child(Some(&vbox));
    popup.set_parent(&scr);
    popup.set_pointing_to(Some(&rect));
    let popup_clone = popup.clone();

    but_cancel.connect_clicked(move |_| {
        popup_clone.popdown();
    });
    let popup_clone = popup.clone();
    but_del.connect_clicked(move |_| {
        popup_clone.popdown();
        delete(
            item.clone(),
            flow_box.clone(),
            history.clone(),
            current_pos.clone(),
            back_button.clone(),
            forward_button.clone(),
            window.clone(),
            scr.clone(),
            copy_memory.clone(),
        )
        .expect("Failed to remove file");
    });

    popup.popup();
}

pub fn is_dark_theme_active() -> bool {
    let mut theme = false;
    // let mut theme_name = String::new();
    if let Some(display) = Display::default() {
        let settings = Settings::for_display(&display);

        let theme_name = settings.gtk_theme_name().unwrap();

        if theme_name.to_lowercase().contains("dark") {
            theme = true;
        }
    } else {
        println!("No default display found.");
    }

    // Default to false (light theme) if no valid result
    theme
}
