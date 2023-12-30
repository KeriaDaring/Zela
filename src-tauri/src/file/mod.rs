use std::ffi::OsStr;
use std::fs;
use std::path::{Path};
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Utc};
use walkdir::DirEntry;



pub struct File {
    entry: DirEntry
}


impl File {
    pub fn from(entry: DirEntry) -> Self{
        File {
            entry
        }
    }

    pub fn get_msg(&self) -> Vec<String> {
        let path: &Path = self.entry.path();

        let name: &OsStr = path.file_name().ok_or("No filename").unwrap();
        let binding = name.to_str().unwrap().to_string();
        let mut list: Vec<_> = binding.split(".").collect();
        let _type = list.pop().expect("获取拓展名错误");

        let metadata = fs::metadata(path).unwrap();
        //len 为字节

        let len = metadata.len();
        let _type: String = if metadata.is_dir() {
            "dir"
        } else if metadata.is_file() {
            _type
        } else if metadata.is_symlink() {
            "link"
        } else {
            "none"
        }.to_string();
        let name = name.to_str().unwrap().to_string();
        let creat = metadata.created().expect("获取创建时间失败");
        let creat = self.format_time(creat);


        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        let time = SystemTime::now() - Duration::from_secs(last_modified.clone());
        let modify = self.format_time(time);

        vec![name, _type, creat, modify, len.to_string()]
    }

    fn format_time(&self, time: SystemTime) -> String {
        DateTime::<Utc>::from(time).format("%Y-%m-%d %H:%M:%S").to_string()
    }

}

