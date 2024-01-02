use std::fs::{create_dir, File};
use std::path::{Path, PathBuf};
use app::file_system::FileSystem;
use app::profile::Profile;


#[derive(Debug)]
pub struct Process {
    user: Profile,
    fs: FileSystem
}



impl Process {
    pub(crate) fn new() -> Self {
        let user = Profile::new();
        let fs = FileSystem::new();
        Process {
            user,
            fs
        }
    }


    // pub fn toggle_ui(&mut self, target: usize) {
    //     self.user.toggle_ui(target);
    // }

    pub fn _move(&mut self, path1: String, path2: String) {
        self.fs._move(path1, path2);
    }

    // pub fn forward(&mut self, id: usize) {
    //     self.fs.pre(id)
    // }

    pub fn search(&mut self, target: &str) {
        self.fs.search(target);
    }

    pub fn access(&mut self, path: String) {
        self.fs.access(PathBuf::from(path))
    }

    pub fn get_file(&mut self) -> Option<Vec<String>> {
        self.fs.get_file()
    }

    pub fn creat(&mut self, path: String, _type: String) {
        let path_in = Path::new(&path);
        match _type.as_str() {
            "dir" => {
                create_dir(path_in);
            }
            _ => {
                File::create(path_in).expect("创建文件失败");
            }
        }
        self.fs.creat(&path);
    }

    pub fn delete(&mut self, path: String) {
        self.fs.delete(path)
    }
    pub fn rename(&self, path: String, new_path: String) {
        self.fs.rename(path, new_path)
    }

    pub fn read_ui(&self) -> Vec<i32> {
        self.user.get_ui()
    }

    pub fn fold(&mut self, target: usize) {
        self.user.toggle_ui(target);
        self.user.print_ui();
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