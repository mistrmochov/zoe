use gtk4::{ApplicationWindow, Button, FlowBox, ScrolledWindow};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::{cd::*, ui_b::*};

pub fn fancy_folder_fallback(
    bad_path: PathBuf,
    flow_box: FlowBox,
    history: Rc<RefCell<Vec<PathBuf>>>,
    current_pos: Rc<RefCell<usize>>,
    back_button: Rc<RefCell<Button>>,
    forward_button: Rc<RefCell<Button>>,
    window: ApplicationWindow,
    scr: ScrolledWindow,
    copy_memory: Rc<RefCell<PathBuf>>,
) {
    if let Some(name) = bad_path.clone().file_name() {
        let problem = format!(
            "The \"{}\" directory doesn't exist or has been moved!",
            name.to_string_lossy()
        );
        error_pop(
            "This directory doesn't exist!".to_string(),
            problem,
            window.clone(),
            scr.clone(),
        );
        if *current_pos.borrow() == 0 {
            return;
        }
    }

    let path_to_navigate = {
        let current_pos_clone = current_pos.borrow_mut();

        let history_clone = history.borrow();
        Some(history_clone[*current_pos_clone - 1].clone()) // Return only the PathBuf
    };
    if let Some(path) = path_to_navigate {
        let mut hist = history.borrow_mut();
        let mut pos = current_pos.borrow_mut();

        hist.truncate(*pos - 1);
        hist.push(path.clone());
        *pos = hist.len() - 1;

        if path.exists() {
            select_folder(
                path.clone(),
                flow_box.clone(),
                history.clone(),
                current_pos.clone(),
                back_button.clone(),
                forward_button.clone(),
                false,
                window.clone(),
                scr.clone(),
                copy_memory.clone(),
            );
        } else {
            fancy_folder_fallback(
                bad_path,
                flow_box.clone(),
                history.clone(),
                current_pos.clone(),
                back_button.clone(),
                forward_button.clone(),
                window.clone(),
                scr.clone(),
                copy_memory.clone(),
            );
        }
    }
}
