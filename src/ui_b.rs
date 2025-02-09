use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use gdk4::Rectangle;
use gtk4::{prelude::*, Orientation, ScrolledWindow};
use gtk4::{ApplicationWindow, Box, Button, FlowBox, Label, Popover};

use crate::cd::cp_dir;
use crate::cd::select_folder;

pub fn empty_space_pop(
    x: f64,
    y: f64,
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    flow_box: FlowBox,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
) {
    let popup_options = vec![
        "New Folder...",
        "Open With...",
        "Paste",
        "Select All",
        "Properties",
    ];
    let popup = Popover::builder().has_arrow(false).build();
    let vbox = Box::new(Orientation::Vertical, 5);

    for option in popup_options.iter() {
        let label = Label::new(Some(option));
        label.set_xalign(0.01);
        label.add_css_class("files_color");
        label.add_css_class("p_but_label");
        let button_pop = Button::builder().child(&label).build();
        let option_clone = option.to_string().clone();
        let copy_memory_clone = copy_memory.clone();
        let current_pos_clone = current_pos.clone();
        let history_clone = history.clone();
        let flow_box_clone = flow_box.clone();
        let back_button_clone = back_button.clone();
        let forward_button_clone = forward_button.clone();
        let window_clone = window.clone();
        let scr_clone = scr.clone();

        button_pop.add_css_class("flat");
        let popup_clone = popup.clone();
        button_pop.connect_clicked(move |_| {
            popup_clone.popdown();
            if option_clone == "Paste" {
                if copy_memory_clone.borrow().exists() {
                    let path_to_navigate = {
                        let current_pos_clone_b = current_pos_clone.borrow_mut();

                        let history_clone_b = history_clone.borrow();
                        Some(history_clone_b[*current_pos_clone_b].clone()) // Return only the PathBuf
                    };
                    if let Some(path) = path_to_navigate {
                        let copy_memory_p = copy_memory_clone.borrow().to_path_buf();
                        if let Some(filename) = copy_memory_p.clone().file_name() {
                            if copy_memory_p.is_dir() {
                                cp_dir(copy_memory_p, path.clone());
                            } else {
                                if path.clone().join(filename).exists() {
                                    fs::remove_file(path.clone().join(filename))
                                        .expect("Failed to remove file.");
                                }
                                fs::copy(copy_memory_p, path.clone().join(filename))
                                    .expect("Failed to copy file");
                            }
                            while let Some(child) = flow_box_clone.first_child() {
                                flow_box_clone.remove(&child);
                            }
                            select_folder(
                                path,
                                flow_box_clone.clone(),
                                history_clone.clone(),
                                current_pos_clone.clone(),
                                back_button_clone.clone(),
                                forward_button_clone.clone(),
                                false,
                                window_clone.clone(),
                                scr_clone.clone(),
                                copy_memory_clone.clone(),
                            );
                        }
                    }
                }
            }
        });
        vbox.append(&button_pop);
        if (copy_memory.borrow().to_string_lossy() == "") && (option.to_string() == "Paste") {
            button_pop.set_sensitive(false);
        }
    }

    popup.set_child(Some(&vbox));

    let wx = x as i32 + 55;
    let wy = y as i32;

    let rect = Rectangle::new(wx, wy, 1, 1);

    popup.set_pointing_to(Some(&rect));

    popup.set_parent(&scr);
    popup.popup();
}
