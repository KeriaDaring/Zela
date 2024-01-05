use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::Metadata;
use std::path::{Path};
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Utc};
use rayon::iter::ParallelBridge;
use serde_json::to_string;
use tantivy::{doc, HasLen, Index};
use walkdir::{DirEntry, WalkDir};



pub
struct File {
    entry: DirEntry
}


impl File {
    pub fn from(entry: DirEntry) -> Self{
        File {
            entry
        }
    }

    pub fn add_in_sql(&self) {
        let index = Index::open_in_dir("index").expect("打开index错误");
        let mut index_writer = index.writer(150000000).expect("writer获取失败");

        let file_info = self.get_msg().unwrap_or_else(|name|
            FileInfo::new(name.0, name.1, name.2));

        let name = index.schema().get_field("name").unwrap();
        let path = index.schema().get_field("path").unwrap();
        let _type = index.schema().get_field("type").unwrap();
        let creat = index.schema().get_field("creat").unwrap();
        let modify = index.schema().get_field("modify").unwrap();
        let size = index.schema().get_field("size").unwrap();
        index_writer.add_document(doc!(
            name => file_info.name,
            path => file_info.path,
            _type => file_info._type,
            creat => file_info.creat,
            modify => file_info.modify,
            size => file_info.size
    )).expect("条目创建错误");
        index_writer.commit().expect("数据库提交失败");
    }

    pub fn get_msg<'a>(&self) -> Result<FileInfo, (String, String, String)> {
        let path: &Path = self.entry.path();
        let name: &OsStr = path.file_name().ok_or("No filename").unwrap();
        let mut name = name.to_str().expect("name 转换失败").to_string();
        let path1 = path.as_os_str().to_str().unwrap().to_string();
        let binding = name.as_str().to_string();
        let mut list: Vec<_> = binding.split(".").collect();
        let _type = list.pop().expect("获取拓展名错误").to_string();

        let metadata: Metadata;
        match fs::metadata(path) {
            Ok(val) => {metadata = val}
            Err(e) => {
                return Err((name, path1.clone(), _type));
            }
        }
        //len 为字节

        let len = metadata.len().to_string();
        // let _type: String = if metadata.is_dir() {
        //     "dir"
        // } else if metadata.is_file() {
        //     _type
        // } else if metadata.is_symlink() {
        //     "link"
        // } else {
        //     "none"
        // };

        let creat = metadata.created().expect("获取创建时间失败");
        let creat = self.format_time(creat);


        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        let time = SystemTime::now() - Duration::from_secs(last_modified.clone());
        let modify = self.format_time(time);

        if name.eq("no") {
            name = path1.clone();
        }

        Ok(FileInfo {
            name,
            path: path1,
            _type,
            creat,
            modify,
            size: len,
        })
    }

    fn format_time(&self, time: SystemTime) -> String {
        DateTime::<Utc>::from(time).format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn msg(&self) -> Vec<String> {
        let path: &Path = self.entry.path();
        let path_str = path.as_os_str().to_str().unwrap().to_string();
        let name: &OsStr;
        #[cfg(target_os = "macos")]
        {
            name = path.file_name().ok_or("No filename").unwrap();
        }

        #[cfg(target_os = "windows")]
        {
            name = match path.file_name() {
                Some(val) => val,
                None => OsStr::new("no"),
            };
        }

        let binding = name.to_str().unwrap().to_string();
        let mut list: Vec<_> = binding.split(".").collect();
        let _type = list.pop().expect("获取拓展名错误");

        let metadata = match fs::metadata(path) {
            Ok(val) => val,
            Err(_) => {
                return vec![name.to_str().unwrap().to_string(), path_str, _type.to_string(), "无".to_string(), "无".to_string(), "无".to_string()];
            }
        };
        //len 为字节

        let mut len = metadata.len().to_string();
        // let len = WalkDir::new(path)
        //     .min_depth(1)
        //     .max_depth(5)
        //     .into_iter()
        //     // .par_bridge()
        //     .filter_map(|entry| entry.ok())
        //     .filter_map(|entry| entry.metadata().ok())
        //     .filter(|metadata| metadata.is_file())
        //     .fold(0, |acc, m| m.len() + acc);
        let _type = if metadata.is_dir() {
            len = "无".to_string();
            "Folder"
        } else if metadata.is_file() {
            _type
        } else if metadata.is_symlink() {
            "link"
        } else {
            "none"
        }.to_string();
        let mut name = name.to_str().expect("name 转换失败").to_string();
        let creat = metadata.created().expect("获取创建时间失败");
        let creat = self.format_time(creat);


        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        let time = SystemTime::now() - Duration::from_secs(last_modified.clone());
        let modify = self.format_time(time);
        let path = path.as_os_str().to_str().unwrap().to_string();
        if name.eq("no") {
            name = path.clone();
        }

        vec![name, path, _type, creat, modify, len.to_string()]
    }

}

struct FileInfo {
    name: String,
    path: String,
    _type: String,
    creat: String,
    modify: String,
    size: String,
}


impl FileInfo {
    pub fn new(name: String, path: String, _type: String) -> Self {
        FileInfo {
            name,
            path,
            _type,
            creat: "无".to_string(),
            modify: "无".to_string(),
            size: "无".to_string(),
        }
    }
}