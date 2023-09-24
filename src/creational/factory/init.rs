use crate::creational::factory::gui::Dialog;
use crate::creational::factory::html_gui::HtmlDialog;
use crate::creational::factory::windows_gui::WindowsDialog;

pub fn initialize() -> &'static dyn Dialog {
    // The dialog type is selected depending on the environment settings or configuration.
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        &HtmlDialog
    }
}
