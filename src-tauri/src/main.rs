#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::ffi::OsString;
use std::fs;
use std::process::Command;
use std::sync::Mutex;
pub mod process;

use app::setup;
use process::{Process as ps};
use crate::process::Process::Process;


#[macro_use]
extern crate lazy_static;


lazy_static! {

    static ref PROCESS: Mutex<Process> = Mutex::new(ps::Process::new());
}


#[tauri::command]
fn pre(id: usize) {
    PROCESS.lock().unwrap().pre(id)
}
#[tauri::command]
fn forward(id: usize) {
    PROCESS.lock().unwrap().forward(id)
}

#[tauri::command]
fn access(path: String) {
    PROCESS.lock().expect("damn").access(path);
}

#[tauri::command]
fn get_file() -> (bool, Vec<String>) {
    PROCESS.lock().expect("获取文件失败").get_file()
}

#[tauri::command]
fn copy(path1: String, path2: String) {
    let source = OsString::from(path1);
    let destination = OsString::from(path2);
    match fs::copy(source, destination) {
        Ok(_) => println!("File copied successfully."),
        Err(e) => println!("Error copying file: {:?}", e),
    }
}
#[tauri::command]
fn _move(path1: String , path2: String) {
    let source = OsString::from(path1);
    let destination = OsString::from(path2);

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "move", source, destination])
        .status()
        .expect("Failed to move file.");

    #[cfg(not(target_os = "windows"))]
    Command::new("mv")
        .args([source.clone(), destination.clone()])
        .status()
        .expect("Failed to move file.");

    PROCESS
        .lock()
        .expect("移动文件失败")
        ._move(source, destination);
}
#[tauri::command]
fn delete(path: String) {

}
#[tauri::command]
fn creat(name: String) {

}

#[tauri::command]
fn test() {
    println!("这是一个测试函数");
}

#[tauri::command]
fn open(path:String) {
    let file_path = OsString::from(path);
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "", file_path])
        .spawn()
        .expect("Failed to open file.");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(file_path)
        .spawn()
        .expect("Failed to open file.");

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(file_path)
        .spawn()
        .expect("Failed to open file.");
}
#[tauri::command]
fn toggle_ui(target: usize) {
    PROCESS.lock().expect("受折").toggle_ui(target);
}

#[tauri::command]
fn read_ui() -> Vec<i32>{
    PROCESS.lock().expect("初始化ui错误").read_ui()
}

#[tauri::command]
fn search(target: String) {
    PROCESS.lock().expect("damn").search(&target);
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test, read_ui, access, _move, open, creat, copy, get_file, search, pre, forward])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



