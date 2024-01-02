#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::ffi::OsString;
use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tauri::utils::html::parse;

pub mod process;

use app::setup;
use crate::process::Process;


#[macro_use]
extern crate lazy_static;


lazy_static! {

    static ref PROCESS: Mutex<Process> = Mutex::new(Process::new());
}


// #[tauri::command]
// fn pre(id: usize) {
//     PROCESS.lock().unwrap().pre(id)
// }
// #[tauri::command]
// fn forward(id: usize) {
//     PROCESS.lock().unwrap().forward(id)
// }

#[tauri::command]
fn access(path: String) {
    PROCESS.lock().expect("damn").access(path);
}

#[tauri::command]
fn get_file() -> Option<Vec<String>> {
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
    let source = OsString::from(path1.clone());
    let destination = OsString::from(path2.clone());

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
        ._move(path1, path2);
}

#[tauri::command]
fn read_ui() -> Vec<i32>{
    PROCESS.lock().expect("操作冲突了").read_ui()
}

#[tauri::command]
fn delete(path: String) {
    PROCESS.lock().expect("操作冲突啦").delete(path);
}
#[tauri::command]
fn creat(path: String, _type: String) {
    PROCESS.lock().expect("操作冲突啦").creat(path, _type)
}

#[tauri::command]
fn rename(path: String, new_name: String) {
    fs::rename(path.clone(), new_name.clone()).expect("重命名失败");
    PROCESS.lock().expect("操作冲突啦").rename(path, new_name)
}

#[tauri::command]
fn fold(target: usize) {
    PROCESS.lock().expect("操作冲突了").fold(target)
}

#[tauri::command]
fn test() -> Vec<String> {
    let mut list = Vec::new();
    list.push("hello".to_string());
    list.push("hello".to_string());
    list.push("hello".to_string());
    list.push("hello".to_string());
    list.push("hello".to_string());
    list

    //验证了可以传数组
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
// #[tauri::command]
// fn toggle_ui(target: usize) {
//     PROCESS.lock().expect("受折").toggle_ui(target);
// }

// #[tauri::command]
// fn read_ui() -> Vec<i32>{
//     PROCESS.lock().expect("初始化ui错误").read_ui()
// }

#[tauri::command]
fn search(target: String) {
    PROCESS.lock().expect("damn！ search 失败").search(&target);
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test,read_ui,fold, access, _move, open, creat, copy, get_file, search])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



