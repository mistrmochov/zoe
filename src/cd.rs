use crate::buttons_check_sensitive;
use crate::ui::pop_up;
use gtk4::prelude::*;
use gtk4::{
    self as gtk, Align, ApplicationWindow, Box, Button, FlowBox, GestureClick, IconSize, Image,
    Label, Orientation, ScrolledWindow,
};
use std::cell::RefCell;
use std::fs::{self};
use std::io::{self};
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
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
) {
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
    let folder_clone = folder.clone();
    let flow_box_clone = flow_box.clone();
    let history_clone = history.clone();
    let copy_memory_clone = copy_memory.clone();

    cd(
        files,
        folder_clone.clone(),
        flow_box_clone.clone(),
        history_clone,
        current_pos.clone(),
        back_button.clone(),
        forward_button.clone(),
        window.clone(),
        scr.clone(),
        copy_memory_clone.clone(),
    );

    cd(
        dotfiles,
        folder.clone(),
        flow_box,
        history.clone(),
        current_pos.clone(),
        back_button.clone(),
        forward_button.clone(),
        window,
        scr,
        copy_memory_clone.clone(),
    );

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

fn cd(
    files: Vec<String>,
    folder: PathBuf,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
) {
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
        let flow_box_clone = flow_box.clone();
        let history_clone = history.clone();
        let current_pos_clone = current_pos.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();
        let button_clone = button.clone();
        let window_clone = window.clone();
        let scr_clone = scr.clone();
        let new_item_clone = new_item.clone();
        let copy_memory_clone = copy_memory.clone();

        gesture.connect_pressed(move |_gesture, n_press, _x, _y| {
            if n_press == 2 {
                if item_path.is_dir() && item_path.exists() {
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
                        scr_clone.clone(),
                        copy_memory_clone.clone(),
                    );
                }
            }
        });
        let window_clone = window.clone();
        let item_clone = item.clone();
        let flow_box_clone = flow_box.clone();
        let history_clone = history.clone();
        let current_pos_clone = current_pos.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();
        let scr_clone = scr.clone();
        let copy_memory_clone = copy_memory.clone();

        gesture_popup.connect_pressed(move |_gesture, n_press, x, y| {
            if n_press == 1 {
                pop_up(
                    button_clone.clone(),
                    x,
                    y,
                    new_item_clone.clone(),
                    item_clone.clone().to_string(),
                    flow_box_clone.clone(),
                    history_clone.clone(),
                    current_pos_clone.clone(),
                    back_button_clone.clone(),
                    forward_button_clone.clone(),
                    window_clone.clone(),
                    scr_clone.clone(),
                    copy_memory_clone.clone(),
                );
            }
        });

        gesture.set_propagation_phase(gtk::PropagationPhase::Capture);
        gesture_popup.set_propagation_phase(gtk::PropagationPhase::Capture);

        // Add the gesture to the button
        button.add_controller(gesture_popup);
        button.add_controller(gesture);
        flow_box.insert(&button, -1);
    });
}

pub fn delete(
    item: String,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
) -> io::Result<()> {
    let item_path = PathBuf::from(item);
    if item_path.exists() {
        if item_path.is_dir() {
            fs::remove_dir_all(item_path)?;
        } else if item_path.is_file() {
            fs::remove_file(item_path)?;
        } else if item_path.is_symlink() {
            fs::remove_file(item_path)?;
        }
    }

    let path_to_navigate = {
        let current_pos_clone = current_pos.borrow_mut();

        let history_clone = history.borrow();
        Some(history_clone[*current_pos_clone].clone()) // Return only the PathBuf
    };
    if let Some(path) = path_to_navigate {
        if path.exists() {
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
                copy_memory,
            );
        }
    }

    Ok(())
}

pub fn cp_dir(source: PathBuf, destination: PathBuf) {
    if source.exists() {
        if let Some(name) = source.file_name() {
            let dest = destination.join(name);
            if !dest.exists() {
                fs::create_dir(dest.clone()).expect("Failed to create directory");
            }
            println!("{}", source.to_string_lossy());

            if let Ok(items) = fs::read_dir(source) {
                for item in items {
                    if let Ok(item) = item {
                        let dest_clone = dest.clone();
                        let item_path = PathBuf::from(item.path());
                        if let Some(item_name) = item_path.file_name() {
                            let item_path_clone = item_path.clone();
                            if item_path.is_dir() {
                                cp_dir(item_path_clone.clone(), dest_clone.clone());
                            } else {
                                fs::copy(item_path_clone, dest_clone.join(item_name))
                                    .expect("Failed to copy file.");
                            }
                        }
                    }
                }
            }
        }
    }
}
