use crate::file::File;
use std::{env, fs, vec};
use std::path::PathBuf;
use std::str::FromStr;
use std::process::Command;
use std::time::SystemTime;
use chrono::{Utc, DateTime};
use rayon::prelude::*;
use walkdir::WalkDir;
use std::collections::VecDeque;
use std::hash::Hash;
use tantivy::{Document, Index, Term};
use tantivy::collector::TopDocs;
use tantivy::query::RegexQuery;
use tantivy::schema::{Field, Schema, STORED, TEXT, Value};


#[derive(Debug)]
pub struct FileSystem {
    stage: Stage,
    path: PathBuf,
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
    home.remove(0);
    home.pop();
    home
}

#[derive(Debug)]
#[derive(PartialEq)]
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

        let tab_arr: [PathBuf; 2] = [initial_page(), PathBuf::new()];
        if fs::metadata(&flag_file_path).is_ok() {
            FileSystem {
                stage: Stage::Done,
                path: home,
                home: init_home(),
                queue: VecDeque::new(),
            }
        } else {
            FileSystem {
                stage: Stage::Done,
                path: home,
                home: init_home(),
                queue: VecDeque::new(),
            }
        }
    }



    pub fn scan_all(&self) {
        let index = Index::create_in_dir("./index", {
            let mut schema_builder = Schema::builder();
            schema_builder.add_text_field("name", TEXT | STORED);
            schema_builder.add_text_field("path", STORED);
            schema_builder.add_text_field("type", TEXT | STORED);
            schema_builder.add_text_field("creat", TEXT | STORED);
            schema_builder.add_text_field("modify", TEXT | STORED);
            let schema = schema_builder.build();
            schema
        }).expect("index 创建失败");
        let _ = &self.home.iter().for_each(|n| WalkDir::new(n)
            // .max_depth(3)
            .into_iter()
            .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        let file = File::from(entry);
                        file.add_in_sql();
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }));
    }

    pub fn _move(&self, path1: String, path2: String) {
        self.creat(path2.as_str());
        self.delete(path1);
    }
    pub fn rename(&self, path: String, new_name: String) {
        self.delete(path);
        self.creat(new_name.as_str())
    }

    pub fn creat(&self, path: &str) {
        let index = Index::open_in_dir("../index").expect("打开index错误");
        WalkDir::new(PathBuf::from(path))
            .max_depth(0)
            .into_iter()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        let file = File::from(entry);
                        file.add_in_sql();
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            });
    }
    pub fn delete(&self, path: String) {
        let index = Index::open_in_dir("../index").expect("打开索引失败");

        let schema = index.schema();
        let field = schema.get_field("path").expect("获取field失败");

        let mut writer = index.writer(5_000_000).expect("获取writer失败");

        let term = Term::from_field_text(field, path.as_str());
        writer.delete_term(term);
        writer.commit().expect("操作提交失败");
    }

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
                        self.queue.push_front(File::from(entry).msg());
                    }
                    Err(err) => eprintln!("访问失败 {}", err),
                }
            });
        self.path = name;
        self.stage = Stage::Done;
    }

    pub fn get_file(&mut self) -> Option<Vec<String>> {
        self.queue.pop_back()
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
        self.clear_queue();

        let index = Index::open_in_dir("../index").unwrap();
        let schema = index.schema();
        let fields = vec![
            schema.get_field("name").unwrap(),
            schema.get_field("path").unwrap(),
            schema.get_field("type").unwrap(),
            schema.get_field("creat").unwrap(),
            schema.get_field("modify").unwrap(),
            schema.get_field("size").unwrap(),
        ];
        let fields1 = vec![
            schema.get_field("name").unwrap(),
            schema.get_field("type").unwrap(),
            schema.get_field("creat").unwrap(),
            schema.get_field("modify").unwrap(),
        ];
        let mut list = Vec::new();


        for i in fields1 {
            self.search_comp(pattern, i, &mut list);
        }

        for i in list {
            let vec = self.transform(i, fields.clone());
            self.queue.push_front(vec);
        }

        for i in &self.queue {
            println!("{:?}", i);
        }
    }

    fn transform(&mut self, doc: Document, field: Vec<Field>) -> Vec<String>{
        let mut list: Vec<String> = Vec::new();
        for i in field {
            let val = doc.get_first(i).expect("拿到属性列表").as_text().expect("拿到属性错误");
            list.push(val.to_string());
        }
        list
    }

    fn search_comp(&self, pattern: &str, field: Field, list: &mut Vec<Document>) {
        let index = Index::open_in_dir("../index").expect("index打开错误");
        let reader = index.reader().expect("reader初始化错误");
        let searcher = reader.searcher();
        let regex_query = RegexQuery::from_pattern(pattern, field).expect("查找器初始化失败");

        // 执行查询
        let top_docs = searcher.search(&regex_query, &TopDocs::with_limit(100000000)).expect("查找失败");

        // 创建一个 Vec 以保存匹配的文档信息
        let mut matched_docs: Vec<tantivy::DocAddress> = Vec::new();

        // 提取匹配的文档信息
        for (_score, doc_address) in top_docs {
            matched_docs.push(doc_address);
        }

        // 处理匹配的文档信息
        for doc_address in matched_docs {
            let retrieved_doc = searcher.doc(doc_address).expect("文档转换");
            // println!("Matched Document: {:?}", retrieved_doc);
            list.push(retrieved_doc);
        }

    }

    // pub fn find_keys_containing_pattern(&mut self, pattern: &str) {
    //     self.queue.clear();
    //     self.stage = Stage::Doing;
    //     // 使用 scan_prefix 方法进行模式匹配
    //     for kv in self.tree.scan_prefix(pattern.as_bytes()) {
    //         if let Ok((key, _value)) = kv {
    //             // 将字节序列的键转换为字符串
    //             if let Ok(key_str) = String::from_utf8(key.to_vec()) {
    //                 self.queue.push_back(vec![key_str]);
    //             }
    //         }
    //     }
    //     self.stage = Stage::Done;
    // }

    // fn rev_path(&self, path: DirEntry) -> String {
    //     let mut path = path.into_path();
    //     let mut container = PathBuf::new();
    //     while let Some(block) = path.file_name() {
    //         container.push(PathBuf::from(block));
    //         path.pop();
    //     }
    //     container.to_str().unwrap().to_string()
    // }
}


#[cfg(test)]
mod test {
    use std::time::Duration;
    use sled::Db;
    use super::*;


    // pub fn find_keys_containing_pattern(db: &Db, pattern: &str) -> Vec<String> {
    //     let mut result_keys = Vec::new();
    //
    //     // 使用 scan_prefix 方法进行模式匹配
    //     for kv in db.scan_prefix(pattern.as_bytes()) {
    //         if let Ok((key, _value)) = kv {
    //             // 将字节序列的键转换为字符串
    //             if let Ok(key_str) = String::from_utf8(key.to_vec()) {
    //                 result_keys.push(key_str);
    //             }
    //         }
    //     }
    //     result_keys
    // }


    // pub fn rev_path(path: DirEntry) -> String {
    //     let mut path = path.into_path();
    //     let mut container = PathBuf::new();
    //     while let Some(block) = path.file_name() {
    //         container.push(PathBuf::from(block));
    //         path.pop();
    //     }
    //     container.to_str().unwrap().to_string()
    // }

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


    // #[test]
    // fn print_tree() {
    //     let db = sled::open("./snapshot")
    //         .expect("数据库打开错误");
    //     println!("{:?}", db);
    // }

    // #[test]
    // fn test_tree() {
    //     let db = sled::open("./snapshot")
    //         .expect("数据库打开错误");
    //     let _ = db.clear();
    //     db.insert("User/hello/Desktop", "Desktop")
    //         .expect("插入错误");
    //     println!("{:?}", db);
    // }

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

    #[test]
    pub fn scan_all() {
        let index = Index::create_in_dir("../index", {
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
        let index = Index::open_in_dir("../index").expect("打开index错误");
        WalkDir::new(env::home_dir().expect("home"))
            .max_depth(2)
            .into_iter()
            // .par_bridge()
            .for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        let file = File::from(entry);
                        file.add_in_sql();
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            });

        let index = Index::open_in_dir("../index").expect("打开index错误");
        println!("{:?}", index)
    }


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
    fn search() {
        let pattern = "2023";
        let mut fs = FileSystem::new();
        fs.search(pattern);
        println!("{:?}", fs.queue)
    }

    #[test]
    fn access() {
        let mut fs = FileSystem::new();
        fs.access(PathBuf::from_str("/Users/keria/Downloads").expect("文件夹访问失败"));
        loop {
            match fs.get_file() {
                None => {
                    if fs.stage == Stage::Done {
                        break;
                    }
                }
                Some(val) => {
                    println!("{:?}", val);
                }
            }
        }
    }

    // #[test]
    // fn find_desktop() {
    //     let tree = sled::open("./snapshot").expect("你好");
    //     let result = find_keys_containing_pattern(&tree, "Desktop");
    //     println!("{:?}", result);
    // }

    // #[test]
    // fn scan_all() {
    //     let home = env::home_dir().unwrap();
    //     let tree = sled::open("./snapshot").expect("你好");
    //     let _ = tree.clear();
    //     WalkDir::new(home)
    //         .max_depth(3)
    //         .into_iter()
    //         .par_bridge()
    //         .for_each(|entry| {
    //             match entry {
    //                 Ok(entry) => {
    //                     tree.insert(rev_path(entry.clone()).as_bytes(),
    //                                 entry.path().to_str().expect("转换错误").as_bytes())
    //                         .expect("数据插入错误");
    //                 }
    //                 Err(err) => eprintln!("Error: {}", err),
    //             }
    //         });
    // }
}