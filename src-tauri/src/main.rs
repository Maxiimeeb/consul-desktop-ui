// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file;

fn main() {
    file::get_file_system_helper().initialize_app_directory().unwrap();

    consul_desktop_ui_lib::run()
}
