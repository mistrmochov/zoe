use crate::buttons_check_sensitive;
use gdk4::Rectangle;
use gtk4::prelude::*;
use gtk4::{
    self as gtk, Align, ApplicationWindow, Box, Button, FlowBox, GestureClick, IconSize, Image,
    Label, ListBox, ListBoxRow, Orientation, Popover,
};
use std::cell::RefCell;
use std::fs::{self};
use std::path::PathBuf;
use std::rc::Rc;

pub fn remove_home_start(item: &str, home: String) -> String {
    item.trim_start_matches(&home).to_string()
}

pub fn select_folder(
    folder: PathBuf,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    modify_history: bool,
    window: ApplicationWindow,
) {
    let flow_box_clone = flow_box.clone();
    let mut files = Vec::new();
    let mut dotfiles = Vec::new();
    if let Ok(entries) = fs::read_dir(&folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                #[cfg(unix)]
                let format_folder = format!("{}/.", folder.to_string_lossy());
                #[cfg(windows)]
                let format_folder = format!("{}\\.", folder.to_string_lossy());
                if !path.to_string_lossy().starts_with(&format_folder) {
                    files.push(path.to_string_lossy().to_string());
                } else {
                    dotfiles.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    files.sort();
    dotfiles.sort();

    let popup = Popover::builder().has_arrow(false).build();
    let popup_options_list = ListBox::new();
    let popup_options = vec!["Open", "Cut", "Copy", "Move", "Rename", "Delete"];

    for item in popup_options {
        let row = ListBoxRow::new();
        let label = Label::new(Some(&item));
        row.set_child(Some(&label));
        popup_options_list.append(&row);
    }

    popup.set_child(Some(&popup_options_list));

    let popup_dot = Popover::builder().has_arrow(false).build();
    let popup_options_list_dot = ListBox::new();
    let popup_options_dot = vec!["Open", "Cut", "Copy", "Move", "Rename", "Delete"];

    for item in popup_options_dot {
        let row = ListBoxRow::new();
        let label = Label::new(Some(&item));
        row.set_child(Some(&label));
        popup_options_list_dot.append(&row);
    }

    popup_dot.set_child(Some(&popup_options_list_dot));

    files.iter().for_each(|item| {
        #[cfg(unix)]
        let format_folder = format!("{}/", folder.to_string_lossy());
        #[cfg(windows)]
        let format_folder = format!("{}\\", folder.to_string_lossy());
        let new_item = remove_home_start(item, format_folder.clone());
        let label = Label::new(Some(&new_item));
        label.set_xalign(0.01);
        label.add_css_class("files_color");
        let item_path = PathBuf::from(&item);
        let icon_name = if item_path.is_dir() {
            "folder"
        } else {
            "text-x-generic"
        };
        let icon = Image::from_icon_name(icon_name);
        icon.set_icon_size(IconSize::Large);
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Start);
        let hbox = Box::new(Orientation::Horizontal, 5);
        hbox.append(&icon);
        hbox.append(&label);
        let button = Button::builder().child(&hbox).build();
        button.add_css_class("files");

        let gesture = GestureClick::new();
        let gesture_popup = GestureClick::new();

        gesture.set_button(1);
        gesture_popup.set_button(3);
        // Ensure left-click only
        let flow_box_clone = flow_box_clone.clone();
        let flow_box_clone2 = flow_box_clone.clone();
        let history_clone = history.clone();
        let current_pos_clone = current_pos.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();
        let popup_clone = popup.clone();
        let button_clone = button.clone();
        let window_clone = window.clone();

        popup.set_parent(&button);

        gesture.connect_pressed(move |_gesture, n_press, _x, _y| {
            if n_press == 2 {
                if item_path.is_dir() {
                    while let Some(child) = flow_box_clone.first_child() {
                        flow_box_clone.remove(&child); // Use reference to child
                    }
                    select_folder(
                        item_path.clone(),
                        flow_box_clone.clone(),
                        history_clone.clone(),
                        current_pos_clone.clone(),
                        back_button_clone.clone(),
                        forward_button_clone.clone(),
                        true,
                        window_clone.clone(),
                    );
                }
            }
        });
        let window_clone = window.clone();
        gesture_popup.connect_pressed(move |_gesture, n_press, _x, _y| {
            if n_press == 1 {
                let alloc = button_clone.clone().allocation();
                let rect = Rectangle::new(
                    (alloc.x() as f64 + _x) as i32,
                    (alloc.y() as f64 + _y) as i32,
                    1, // Width of the pointer "target" area
                    1, // Height of the pointer "target" area
                );

                popup_clone.set_pointing_to(Some(&rect));
                popup_clone.popup();
            }
        });

        // Prevent the Button widget from consuming the click event
        gesture.set_propagation_phase(gtk::PropagationPhase::Capture);
        gesture_popup.set_propagation_phase(gtk::PropagationPhase::Capture);

        // Add the gesture to the button
        button.add_controller(gesture_popup);
        button.add_controller(gesture);
        flow_box.insert(&button, -1);
    });

    dotfiles.iter().for_each(|item| {
        #[cfg(unix)]
        let format_folder = format!("{}/", folder.to_string_lossy());
        #[cfg(windows)]
        let format_folder = format!("{}\\", folder.to_string_lossy());
        let new_item = remove_home_start(item, format_folder.clone());
        let label = Label::new(Some(&new_item));
        label.set_xalign(0.0);
        let item_path = PathBuf::from(&item);
        let icon_name = if item_path.is_dir() {
            "folder"
        } else {
            "text-x-generic"
        };
        let icon = Image::from_icon_name(icon_name);
        icon.set_icon_size(IconSize::Large);
        icon.set_valign(Align::Center);
        icon.set_halign(Align::Start);
        let hbox = Box::new(Orientation::Horizontal, 5);
        hbox.append(&icon);
        hbox.append(&label);
        let button = Button::builder().child(&hbox).build();

        let gesture = GestureClick::new();
        let gesture_popup = GestureClick::new();

        gesture.set_button(1); // Ensure left-click only
        gesture_popup.set_button(3);

        let flow_box_clone = flow_box_clone.clone();
        let history_clone = history.clone();
        let current_pos_clone = current_pos.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();
        let popup_clone = popup.clone();
        let window_clone = window.clone();
        button.add_css_class("files");
        gesture.connect_pressed(move |_gesture, n_press, _x, _y| {
            if n_press == 2 {
                if item_path.is_dir() {
                    while let Some(child) = flow_box_clone.first_child() {
                        flow_box_clone.remove(&child); // Use reference to child
                    }
                    select_folder(
                        item_path.clone(),
                        flow_box_clone.clone(),
                        history_clone.clone(),
                        current_pos_clone.clone(),
                        back_button_clone.clone(),
                        forward_button_clone.clone(),
                        true,
                        window_clone.clone(),
                    );
                }
            }
        });

        gesture_popup.connect_pressed(move |_gesture, n_press, _x, _y| {
            if n_press == 1 {
                popup_clone.popup();
            }
        });

        // Prevent the Button widget from consuming the click event
        gesture.set_propagation_phase(gtk::PropagationPhase::Capture);
        gesture_popup.set_propagation_phase(gtk::PropagationPhase::Capture);

        // Add the gesture to the button
        button.add_controller(gesture_popup);
        button.add_controller(gesture);
        flow_box.insert(&button, -1);
    });

    if modify_history {
        let mut hist = history.borrow_mut();
        let mut pos = current_pos.borrow_mut();

        // **Truncate forward history** to prevent jumping between unrelated paths
        hist.truncate(*pos + 1);
        hist.push(folder.clone());
        *pos = hist.len() - 1;
    }

    let pos = *current_pos.borrow();
    let hist_len = history.borrow().len();
    back_button.borrow().set_sensitive(pos > 0);
    forward_button.borrow().set_sensitive(pos < hist_len - 1);
    buttons_check_sensitive(back_button.clone(), forward_button.clone());
}
