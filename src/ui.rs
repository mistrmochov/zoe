use dirs::home_dir;
use gdk4::Rectangle;
use gtk4::{self as gtk, Box, Button, EventControllerMotion, HeaderBar, Image, Label, Popover};
use gtk4::{prelude::*, ApplicationWindow, FlowBox, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::buttons_check_sensitive;
use crate::cd::delete;
use crate::is_hyprland;
use crate::select_folder;

pub fn build_ui(app: &gtk::Application) {
    if let Some(home) = home_dir() {
        let history = Rc::new(RefCell::new(Vec::new()));
        let current_pos = Rc::new(RefCell::new(0usize));

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

        #[cfg(unix)]
        let icons = PathBuf::from("/usr/share/zoe/icons/");
        #[cfg(windows)]
        let icons = PathBuf::from("C:\\Program Files\\zoe\\icons");
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
        back_button.borrow().set_size_request(1, 1);
        forward_button.borrow().set_size_request(1, 1);
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
        let window_clone2 = window_clone.clone();
        let scrolled_window_clone = scrolled_window.clone();

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
        );

        window.set_titlebar(Some(&header_bar));
        window.set_child(Some(&scrolled_window));
        app.connect_activate(move |_| {
            window.present();
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
                let history_clone = history.clone();
                let current_pos_clone = current_pos.clone();
                let flow_box_clone = flow_box.clone();
                let back_button_clone = back_button.clone();
                let forward_button_clone = forward_button.clone();

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

                move |_| {
                    // Use a shorter scope for borrowing
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
                                window_clone2.clone(),
                                scrolled_window_clone.clone(),
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
) {
    let popup = Popover::builder().has_arrow(false).build();
    let vbox = Box::new(Orientation::Vertical, 5);
    let popup_options = vec!["Open", "Cut", "Copy", "Move", "Rename", "Delete"];

    for option in popup_options.iter() {
        let label = Label::new(Some(option));
        label.set_xalign(0.01);
        label.add_css_class("files_color");
        label.add_css_class("p_but_label");
        let button = Button::builder().child(&label).build();
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
        button.connect_clicked(move |_| {
            popup_clone.popdown();
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
                );
            }
        });

        vbox.append(&button);
    }

    popup.set_child(Some(&vbox));

    let rect = button.compute_bounds(&window); // Get absolute position

    let wx = rect.unwrap().x() as i32 + x as i32 + 30;
    let wy = rect.unwrap().x() as i32 + y as i32 - 15;

    let rect = Rectangle::new(wx, wy, 1, 1);
    // println!("Cur - {} : {}", bx, y);
    popup.set_pointing_to(Some(&rect));

    popup.set_parent(&button);
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
) {
    let popup = Popover::builder().has_arrow(false).build();
    let mes = format!("Permanently Delete \"{}\"?", new_item);
    let title = Label::new(Some(&mes));
    title.add_css_class("del_dialog_title");
    let message = Label::new(Some("Permanently deleted items can't be restored"));
    let but_cancel = Button::builder().label("Cancel").build();
    let but_del = Button::builder().label("Delete").build();

    let hbox = Box::new(Orientation::Horizontal, 20);
    hbox.append(&but_cancel);
    hbox.append(&but_del);
    hbox.set_halign(gtk4::Align::Center);

    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.append(&title);
    vbox.append(&message);
    vbox.append(&hbox);
    vbox.set_valign(gtk4::Align::Center);
    vbox.add_css_class("del_dialog_vbox");

    let x = window.width() / 2;
    let y = (window.height() / 2) - 50;
    let rect = Rectangle::new(x, y, 1, 1);
    popup.set_child(Some(&vbox));
    popup.set_parent(&window);
    popup.set_pointing_to(Some(&rect));
    popup.add_css_class("del_dialog");
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
        )
        .expect("Failed to remove file");
    });

    popup.popup();
}
