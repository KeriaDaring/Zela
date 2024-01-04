#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::ffi::OsString;
use std::{env, fs, path};
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rayon::iter::ParallelBridge;
use tantivy::Index;
use tantivy::schema::{Schema, STORED, TEXT};
use app::process::Process;
use tokio::task;
use walkdir::WalkDir;
use rayon::iter::ParallelIterator;
use std::sync::Arc;
use std::thread;
use winsafe::co::KNOWNFOLDERID;
use winsafe::{co, SHGetKnownFolderPath};

#[cfg(target_os = "windows")]
use winres;


pub mod setup;
pub mod file;
use file::{File as File1};


#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref PROCESS: Arc<Mutex<Process>> = Arc::new(Mutex::new(Process::new()));
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
    println!("test access{:?}", path);
    let mut pro_arc = Arc::clone(&PROCESS);
    pro_arc.lock().expect("damn").access(path_build(path));
}

#[tauri::command]
fn get_file() -> Option<Vec<String>> {
    let mut pro_arc = Arc::clone(&PROCESS);
    let a = pro_arc.lock().expect("获取文件失败").get_file();
    a
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
    .args(["/C", "move", path_build(path1.clone()).to_str().expect("wrong"), path_build(path2.clone()).to_str().expect("wrong")])
        .status()
        .expect("Failed to move file.");

    #[cfg(not(target_os = "windows"))]
    Command::new("mv")
        .args([source.clone(), destination.clone()])
        .status()
        .expect("Failed to move file.");

        let mut pro_arc = Arc::clone(&PROCESS);
    pro_arc
        .lock()
        .expect("移动文件失败")
        ._move(path_build(path1), path_build(path2));
}

#[tauri::command]
fn read_ui() -> Vec<i32>{
    let mut pro_arc = Arc::clone(&PROCESS);
    let a = pro_arc.lock().expect("操作冲突了").read_ui();
    a
}

#[tauri::command]
fn delete_file(path: Vec<String>) {
    println!("这是接收到的文件vec{:?}", path);
    fs::remove_file(path_build(path.clone())).expect("删除失败");
    let mut pro_arc = Arc::clone(&PROCESS);
    pro_arc.lock().expect("操作冲突啦").delete(path_build(path));
}

#[tauri::command]
fn delete_dir(path: Vec<String>) {
    println!("这是接收到的文件夹vec{:?}", path);
    fs::remove_dir_all(path_build(path.clone())).expect("递归删除文件夹失败");
    let mut pro_arc = Arc::clone(&PROCESS);
    pro_arc.lock().expect("操作冲突啦").delete(path_build(path));
}
#[tauri::command]
fn creat(path: Vec<String>, _type: String) {
    let mut pro_arc = Arc::clone(&PROCESS);
    pro_arc.lock().expect("操作冲突啦").creat(path_build(path), _type);
}

#[tauri::command]
fn rename(path: Vec<String>, new_name: Vec<String>) {
    println!("重命名成功了");
    let mut pro_arc = Arc::clone(&PROCESS);
    fs::rename(Path::new(&path_build(path.clone())), Path::new(&path_build(new_name.clone()))).expect("重命名失败");
    pro_arc.lock().expect("操作冲突啦").rename(path_build(path), path_build(new_name));
}

#[tauri::command]
fn new_dir(path: Vec<String>) {
    println!("new dir {:?}", path);
    fs::create_dir(path_build(path));
    println!("yes_dir")
}

#[tauri::command]
fn new_file(path: Vec<String>) {
    println!("new file {:?}", path);
    File::create(path_build(path));
    println!("yes_file")
}

#[tauri::command]
fn fold(target: usize) {
    let mut pro_arc = Arc::clone(&PROCESS);
    pro_arc.lock().expect("操作冲突了").fold(target - 1);
}

// #[tauri::command]
// fn get_resent() {
    
// }

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
        .args(["/C", "start", "", path_build(path.clone()).to_str().expect("wrong")])
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
    let task = thread::spawn(move || {
        let mut pro_arc = Arc::clone(&PROCESS);
         pro_arc.lock().expect("damn！ search 失败").search(&target);
        });
}

fn path_build(list: Vec<String>) -> PathBuf {
    println!("source {:?}", list);
    let mut path = PathBuf::new();
    let mut stage = false;

    for i in list {
        if !stage {
            path.push(i.clone());
            if cfg!(target_os = "windows") {
                path.push("\\");
            }
            stage = true;
            continue
        }
        path.push(i.clone());
    }
    println!("合并完成了 {:?}", path.clone());
    path
}

fn path_debuild(path: PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

#[tauri::command]
async fn init_tiles() -> Vec<String> {
    let pro_arc = Arc::clone(&PROCESS);
    let list = pro_arc.lock()
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
    let pro_arc = Arc::clone(&PROCESS);
    let list = thread::spawn(move || {
     pro_arc
     .lock()
     .expect("添加磁铁失败")
     .add_tiles(path_build(path));
    });
}

#[tauri::command]
async fn remove_tiles(target: usize) {
    let pro_arc = Arc::clone(&PROCESS);
    let list = thread::spawn(move || {
        pro_arc
        .lock()
        .expect("删除磁贴失败")
        .remove_tiles(target);
    });
}

pub async fn init_index() {
    let mut home = vec![];
    #[cfg(target_os = "macos")]
    home.push(env::home_dir().expect("根目录读取错误"));

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("wmic")
            .args(["logicaldisk", "get", "name"])
            .output()
            .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            // println!("{}", line.trim());
            home.push(PathBuf::from(line.trim()));
        }
    }
    Index::create_in_dir("index", {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("name", TEXT | STORED);
        schema_builder.add_text_field("path", STORED);
        schema_builder.add_text_field("type", TEXT | STORED);
        schema_builder.add_text_field("creat", TEXT | STORED);
        schema_builder.add_text_field("modify", TEXT | STORED);
        schema_builder.add_text_field("size", STORED);
        let schema = schema_builder.build();
        schema
    }).expect("index 创建失败");
    for i in home {
        WalkDir::new(i)
            .max_depth(8)
            .into_iter()
            .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        let file = File1::from(entry);
                        file.add_in_sql();
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            });
    }


}
// #[tokio::main]
fn main() {
    use std::io::Write;
// // only build the resource for release builds
// // as calling rc.exe might be slow
//     #[cfg(target_os = "windows")] {
//     if std::env::var ("PROFILE").unwrap() == "release" {
//         let mut res = winres::WindowsResource::new();
//         res.set_icon("resources\\ico\\fiscalidade_server.ico")
//         .set_manifest(
//             r#"'
// <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
// <trustIno xmlns="urn:schemas-microsoft-com: asm.v3">
// ‹security>
// ‹requestedPrivileges>
// <requestedExecutionLevel level="requireAdministrator" uiAccess="
// </requestedPrivileges>
// </security>
// </trustInfo>
// </assembly>
// "#,
//         );
//         match res.compile() {
//             Err(error) => {
//                 write!(std::io::stderr(), "{}", error).unwrap();
//                 std::process::exit (1);
//             }
//             Ok(_) => {

//             }
//         }
//     }
// }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rename, new_file, new_dir, delete_file, delete_dir, init_tiles, add_tiles, remove_tiles, search, test, read_ui, fold, access, _move, open, creat, copy, get_file])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let scan = task::spawn(init_index());
}



mod test {
    use super::*;

    #[test]
    fn test_resent() {
        // let mut pro_arc = Arc::clone(&PROCESS);
        let docs_folder = SHGetKnownFolderPath(
            &co::KNOWNFOLDERID::Recent,
            co::KF::DEFAULT,
            None,
        ).unwrap();
    println!("{}", docs_folder);
    for path in WalkDir::new(docs_folder) {
        println!("{}", path.unwrap().path().display())
    }
    }
}

