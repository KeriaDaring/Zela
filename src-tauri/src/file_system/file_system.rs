
use crate::file::File;
use std::{env, fs};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;
use chrono::{Utc, DateTime};
use sled::{Config, Db, Mode};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};
use std::collections::VecDeque;


#[derive(Debug)]
pub struct FileSystem {
    tab: Vec<[PathBuf;2]>,
    stage: Stage,
    path: PathBuf,
    tree: Db,
    home: Vec<PathBuf>,
    queue: VecDeque<Vec<String>>,
}

fn initial_page() -> PathBuf {
    let result: PathBuf;

    #[cfg(target_os = "macos")]
    {
        result = env::home_dir().expect("根目录读取错误");
    }


    #[cfg(target_os = "windows")]
    {
        result = PathBuf::new();
    }

    result
}

fn init_home() -> Vec<PathBuf> {
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
    home
}

#[derive(Debug)]
enum Stage {
    Done,
    Doing,
}


impl FileSystem {
    pub fn new() -> Self {
        let mut home: PathBuf = PathBuf::new();
        if let Some(val) = std::env::home_dir() {
            home = val;
        }

        let flag_file_path = "./snapshot";

        let tab_arr: [PathBuf;2] = [initial_page(), PathBuf::new()];
        let vec = vec![tab_arr];
        if fs::metadata(&flag_file_path).is_ok() {
            FileSystem {
                tab: vec,
                stage: Stage::Done,
                path: home,
                tree: sled::open("./snapshot").expect("数据库创建错误"),
                home: init_home(),
                queue:  VecDeque::new(),
            }
        } else {
            FileSystem {
                tab: vec,
                stage: Stage::Done,
                path: home,
                tree: Config::new()
                    .mode(Mode::HighThroughput)
                    .path("./snapshot")
                    .open()
                    .expect("数据库创建错误"),
                home: init_home(),
                queue: VecDeque::new()
            }
        }
    }
    pub fn pre(&mut self, id: usize) {

    }

    pub fn forward(&mut self, id: usize) {

    }


    fn scan_all(&self) {
        let tree = sled::open("./snapshot").expect("你好");
        let _ = tree.clear();
        let _ = &self.home.iter().for_each(|n|WalkDir::new(n)
            // .max_depth(3)
            .into_iter()
            .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        tree.insert(self.rev_path(entry.clone()).as_bytes(),
                                    b"0")
                            .expect("数据插入错误");
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }));
    }

    pub fn _move(&self, path1: PathBuf, path2: PathBuf) {
        self.creat(path2.to_str().expect("creat获取path错误"));
        // self.delete(path1);
    }

    pub fn creat(&self, path: &str) {
        self.tree.insert(path, b"0").expect("内部插入错误");
    }
    // pub fn delete(&self, path: PathBuf) {
    //     let path = path.components().rev().collect::<PathBuf>().as_os_str();
    //     self.tree.remove(path)
    //         .expect("内部删除转换错误");
    // }

    pub fn search(&mut self, pattern: &str) {
        self.find_keys_containing_pattern(pattern);
    }

    pub fn access(&mut self, name: PathBuf) {
        self.clear_queue();
        self.stage = Stage::Doing;
        WalkDir::new(name.clone())
            .max_depth(1)
            .into_iter()
            // .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        self.queue.push_front(File::from(entry).get_msg());
                    }
                    Err(err) => eprintln!("访问失败 {}", err),
                }
            });
        self.path = name;
        self.stage = Stage::Done;
    }

    pub fn get_file(&mut self) -> (bool, Option<Vec<String>>) {
        let stage = match self.stage {
            Stage::Done => true,
            Stage::Doing => false
        };
       match self.queue.pop_back() {
           None => (stage, None),
           Some(val) => (stage, Some(val))
       }
    }

    pub fn toggle_stage(&mut self) {
        match self.stage {
            Stage::Done => {
                self.stage = Stage::Doing;
            }
            Stage::Doing => {
                self.stage = Stage::Done;
            }
        }
    }

    fn clear_queue(&mut self) {
        self.queue.clear();
    }

    pub fn find_keys_containing_pattern(&mut self, pattern: &str) {
        self.queue.clear();
        self.stage = Stage::Doing;
        // 使用 scan_prefix 方法进行模式匹配
        for kv in self.tree.scan_prefix(pattern.as_bytes()) {
            if let Ok((key, _value)) = kv {
                // 将字节序列的键转换为字符串
                if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                    self.queue.push_back(vec![key_str]);
                }
            }
        }
        self.stage = Stage::Done;
    }

    fn rev_path(&self, path: DirEntry) -> String {
        let mut path = path.into_path();
        let mut container = PathBuf::new();
        while let Some(block) = path.file_name() {
            container.push(PathBuf::from(block));
            path.pop();
        }
        container.to_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::*;


    pub fn find_keys_containing_pattern(db: &Db, pattern: &str) -> Vec<String> {
        let mut result_keys = Vec::new();

        // 使用 scan_prefix 方法进行模式匹配
        for kv in db.scan_prefix(pattern.as_bytes()) {
            if let Ok((key, _value)) = kv {
                // 将字节序列的键转换为字符串
                if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                    result_keys.push(key_str);
                }
            }
        }
        result_keys
    }


    pub fn rev_path(path: DirEntry) -> String {
        let mut path = path.into_path();
        let mut container = PathBuf::new();
        while let Some(block) = path.file_name() {
            container.push(PathBuf::from(block));
            path.pop();
        }
        container.to_str().unwrap().to_string()
    }

    // pub fn rev_str(path: PathBuf) -> &'static str {
    //
    // }


    #[test]
    fn home() {
        let home = std::env::home_dir();
        match home {
            None => {}
            Some(val) => {
                println!("{:?}", val.to_str());
            }
        }
    }



    #[test]
    fn print_tree() {
        let db = sled::open("./snapshot")
            .expect("数据库打开错误");
        println!("{:?}", db);
    }

    #[test]
    fn test_tree() {
        let db = sled::open("./snapshot")
            .expect("数据库打开错误");
        let _ = db.clear();
        db.insert("User/hello/Desktop", "Desktop")
            .expect("插入错误");
        println!("{:?}", db);
    }

    fn format_time(time: SystemTime) -> String {
        DateTime::<Utc>::from(time).format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[test]
    fn resent_dir_or_file() {
        let current_dir = env::home_dir().unwrap();
        println!(
            "Entries modified in the last 24 hours in {:?}:",
            current_dir
        );

        for entry in fs::read_dir(current_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = fs::metadata(&path).unwrap();
            //len 为字节


            let len = metadata.len();
            let _type = metadata.file_type();
            let creat = metadata.created().expect("获取创建时间失败");
            let creat = format_time(creat);
            //格式化时间四步骤
            let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
            let time = SystemTime::now() - Duration::from_secs(last_modified.clone());
            let time = format_time(time);

            //最后修改时间

            if last_modified < 24 * 3600 && metadata.is_file() {
                println!(
                    "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                    time,
                    metadata.permissions().readonly(),
                    metadata.len(),
                    path.file_name().ok_or("No filename").unwrap()
                );
            }
        }
    }


    // #[test]
    // fn serialize_tree() {
    //     // let db = Config::new().mode(Mode::HighThroughput).path("./snapshot").open().unwrap();
    //     let db = sled::open("./snapshot").expect("打开数据库错误");
    //     println!("{:?}", db);
    //
    //     db.clear().expect("数据库清楚失败");
    //     println!("{:?}", db);
    //
    //     db.insert(rev_path(PathBuf::from_str("User/keria/Desktop")
    //         .unwrap().borrow_mut()).as_bytes(), "Desktop").expect("插入错误");
    //     db.insert(rev_path(PathBuf::from_str("User/hellonihao/nihao/keria/Desktop")
    //         .unwrap().borrow_mut()).as_bytes(), "Desktop").expect("插入错误");
    //     println!("{:?}", db);
    //
    //     let a = find_keys_containing_pattern(&db, "Desktop");
    //     println!("{:?}", a);
    // }


    //遍历整个文件系统，可以考虑在实现的时候采用多线程
    #[test]
    fn traverse() {
        let home = env::home_dir().unwrap();
        WalkDir::new(home)
            .max_depth(3)
            .into_iter()
            .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_dir() {
                            println!("#{} -------------------------------", entry.file_name().to_str().unwrap().trim());
                            return;
                        }
                        println!("{}", entry.file_name().to_str().unwrap().trim());
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            });
    }

    #[test]
    fn find_desktop() {
        let tree = sled::open("./snapshot").expect("你好");
        let result = find_keys_containing_pattern(&tree, "Desktop");
        println!("{:?}", result);
    }

    #[test]
    fn scan_all() {
        let home = env::home_dir().unwrap();
        let tree = sled::open("./snapshot").expect("你好");
        let _ = tree.clear();
        WalkDir::new(home)
            .max_depth(3)
            .into_iter()
            .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        tree.insert(rev_path(entry.clone()).as_bytes(),
                                    entry.path().to_str().expect("转换错误").as_bytes())
                            .expect("数据插入错误");
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            });
    }
}