use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use app::file_system::file_system::FileSystem;
use app::profile::profile::Profile;

#[derive(Debug)]
pub struct Process {
    user: Profile,
    fs: FileSystem
}



impl Process {
    pub(crate) fn new() -> Self {
        Process {
            user: Profile::new(),
            fs: FileSystem::new(),
        }
    }


    pub fn toggle_ui(&mut self, target: usize) {
        self.user.toggle_ui(target);
    }
    pub fn _move(&mut self, path1: OsString, path2: OsString) {
        self.fs._move(PathBuf::from(path1), PathBuf::from(path2));
    }

    pub fn pre(&mut self, id: usize) {
        self.fs.pre(id);
    }
    pub fn forward(&mut self, id: usize) {
        self.fs.pre(id)
    }
    pub fn read_ui(&self) -> Vec<i32> {
        self.user.get_ui()
    }

    pub fn search(&mut self, target: &str) {
        self.fs.search(target);
    }

    pub fn access(&mut self, path: String) {
        self.fs.access(PathBuf::from(path))
    }

    pub fn get_file(&mut self) -> (bool, Vec<String>) {
        let mut result = (true, Vec::new());
        match self.fs.get_file() {
            (true, None) => {
                self.fs.toggle_stage();
            }
            (false, Some(val)) => {
                result = (false, val);
            }
            _ => {}
        };
        result
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