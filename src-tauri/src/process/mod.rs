use std::env;
use std::fs::{create_dir, File};
use std::io::BufRead;
use std::path::PathBuf;
use tantivy::Opstamp;
use crate::file_system::FileSystem;
use crate::profile::Profile;
use std::process::Command;


#[derive(Debug)]
pub struct Process {
    user: Profile,
    fs: FileSystem
}



impl Process {
    pub fn new() -> Self {
        let user = Profile::new();
        user.print_ui();
        let fs = FileSystem::new();
        Process {
            user,
            fs
        }
    }


    // pub fn toggle_ui(&mut self, target: usize) {
    //     self.user.toggle_ui(target);
    // }

    pub fn _move(&mut self, path1: PathBuf, path2: PathBuf) {
        self.fs._move(path1, path2);
    }

    // pub fn forward(&mut self, id: usize) {
    //     self.fs.pre(id)
    // }

    pub fn search(&mut self, target: &str) {
        self.fs.search(target);
    }

    pub fn access(&mut self, path: PathBuf) {
        self.fs.access(path)
    }pub fn access1(&mut self, path: PathBuf) {
        self.fs.access(path)
    }

    pub fn get_file(&mut self) -> Option<Vec<String>> {
        self.fs.get_file()
    }
    pub fn get_file1(&mut self) -> Option<Vec<String>> {
        self.fs.get_file1()
    }

    pub fn creat(&mut self, path: PathBuf, _type: String) {
        let path_in = path.clone();

        //可以添加更多文件类型
        match _type.as_str() {
            "dir" => {
                create_dir(path_in).expect("文件夹创建失败");
            }
            _ => {
                File::create(path_in).expect("创建文件失败");
            }
        }
        self.fs.creat(path);
    }

    pub fn delete(&mut self, path: PathBuf) {
        match self.fs.delete(path) {
            _ => {}
        }
    }
    pub fn rename(&self, path: PathBuf, new_path: PathBuf) {
        self.fs.rename(path, new_path)
    }

    pub fn read_ui(&self) -> Vec<i32> {
        self.user.get_ui()
    }

    pub fn fold(&mut self, target: usize) {
        self.user.toggle_ui(target);
        self.user.print_ui();
    }

    pub fn init_tiles(&self) -> Vec<PathBuf>{
        match self.user.init_tiles() {
            None => {
                println!("空预设");
                vec![PathBuf::new()]
            }
            Some(val) => {
                val
            }
        }
    }
    pub fn add_tiles(&mut self, path: PathBuf) {
        self.user.add_tiles(path);
    }

    pub fn remove_tiles(&mut self, target: usize) {
        self.user.remove_tiles(target);
    }


    pub fn save(&self) {
        self.user.save();
    }


}


#[cfg(test)]
mod test {
    use super::*;


    fn foundation() {

    }

    fn access_home() {

    }

    fn go_forward() {

    }
}