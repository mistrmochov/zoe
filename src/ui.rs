use dirs::home_dir;
use gtk4::prelude::*;
use gtk4::{self as gtk, Button, EventControllerMotion, HeaderBar, Image};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::buttons_check_sensitive;
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

        let icons = PathBuf::from("/usr/share/zoe/icons/");
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
            .child(&flow_box)
            .build();

        select_folder(
            home,
            flow_box_clone,
            history.clone(),
            current_pos.clone(),
            back_button.clone(),
            forward_button.clone(),
            true,
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
