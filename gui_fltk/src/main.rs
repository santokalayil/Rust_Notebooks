// #![windows_subsystem = "windows"]  // to hide console window

mod gui;
mod cmd_utilities;

// use these if need to embed image
#[macro_use]
extern crate rust_embed;


fn main() {
    cmd_utilities::run_command();
    gui::start_app()
}