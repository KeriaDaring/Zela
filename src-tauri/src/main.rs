#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use lazy_static::lazy_static;
use app::process::Process;


pub mod setup;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref PROCESS: Mutex<Process> = Mutex::new(Process::new());
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
fn access(path: Vec<String>) {
    // println!("{:?}", path);
    PROCESS.lock().expect("damn").access(path_build(path));
}

#[tauri::command]
fn get_file() -> Option<Vec<String>> {
    PROCESS.lock().expect("获取文件失败").get_file()
}

#[tauri::command]
fn copy(path1: Vec<String>, path2: Vec<String>) {
    let source = OsString::from(path_build(path1));
    let destination = OsString::from(path_build(path2));
    match fs::copy(source, destination) {
        Ok(_) => println!("File copied successfully."),
        Err(e) => println!("Error copying file: {:?}", e),
    }
}



#[tauri::command]
fn _move(path1: Vec<String> , path2: Vec<String>) {
    let source = OsString::from(path_build(path1.clone()));
    let destination = OsString::from(path_build(path2.clone()));

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "move", path1.as_str(), path2.as_str()])
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
        ._move(path_build(path1), path_build(path2));
}

#[tauri::command]
fn read_ui() -> Vec<i32>{
    PROCESS.lock().expect("操作冲突了").read_ui()
}

#[tauri::command]
fn delete_file(path: Vec<String>) {
    println!("{:?}", path);
    fs::remove_file(path_build(path.clone())).expect("删除失败");
    PROCESS.lock().expect("操作冲突啦").delete(path_build(path));
}

#[tauri::command]
fn delete_dir(path: Vec<String>) {
    println!("{:?}", path);
    fs::remove_dir_all(path_build(path.clone())).expect("递归删除文件夹失败");
    PROCESS.lock().expect("操作冲突啦").delete(path_build(path));
}
#[tauri::command]
fn creat(path: Vec<String>, _type: String) {
    PROCESS.lock().expect("操作冲突啦").creat(path_build(path), _type)
}

#[tauri::command]
fn rename(path: Vec<String>, new_name: Vec<String>) {
    fs::rename(Path::new(&path_build(path.clone())), Path::new(&path_build(new_name.clone()))).expect("重命名失败");
    PROCESS.lock().expect("操作冲突啦").rename(path_build(path), path_build(new_name))
}

#[tauri::command]
fn fold(target: usize) {
    PROCESS.lock().expect("操作冲突了").fold(target - 1)
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
fn open(path: Vec<String>) {
    let file_path = OsString::from(path_build(path.clone()));
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "", path.as_str()])
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
    // println!("{:?}", target);
    PROCESS.lock().expect("damn！ search 失败").search(&target);
}

fn path_build(list: Vec<String>) -> PathBuf {
    let mut path = if cfg!(target_os = "windows") {
        PathBuf::from("")
    } else {
        PathBuf::from("/")
    };
    for i in list.iter() {
        path.push(i);
    }
    path
}

fn path_debuild(path: PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

#[tauri::command]
async fn init_index() {
    PROCESS.lock().expect("初始化index").init_index();
}

#[tauri::command]
async fn init_tiles() -> Vec<String> {
    let list = PROCESS.lock()
        .expect("初始化磁贴")
        .init_tiles()
        .into_iter()
        .map(|n| path_debuild(n))
        .collect();
    println!("{:?}", list);
    list

}

#[tauri::command]
async fn add_tiles(path: Vec<String>) {
    PROCESS.lock().expect("添加磁铁失败").add_tiles(path_build(path))
}

#[tauri::command]
async fn remove_tiles(target: usize) {
    PROCESS.lock().expect("删除磁贴失败").remove_tiles(target);
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![delete_file, delete_dir, init_tiles, add_tiles, remove_tiles, search, test, read_ui, fold, access, _move, open, creat, copy, get_file])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



