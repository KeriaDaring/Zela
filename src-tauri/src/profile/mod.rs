
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    name: String,
    ui_stage: Vec<i32>,
    tiles: Vec<PathBuf>,
}


impl Profile {
    pub fn new() -> Self {
        let home = std::env::home_dir().unwrap();
        let result = Profile {
            name: String::from("profile"),
            ui_stage: vec![0, 1, 1, 0],
            tiles: vec![home],
        };
        match result.init() {
            Ok(val) => from_str(&*val).unwrap_or_else(|_| result),
            Err(_) => result
        }
    }
    pub fn toggle_ui(&mut self, target: usize) {
        if target == 2  {
            self.ui_stage[3] = 0;
            self.ui_stage[target] = 1;
            return;
        }
        if target == 3 {
            self.ui_stage[2] = 0;
            self.ui_stage[target] = 1;
            return;
        }
        if self.ui_stage[target] == 0 {
            self.ui_stage[target] = 1;
            return;
        }
        self.ui_stage[target] = 0;
    }

    pub fn print_ui(&self) {
        println!("{:?}", self.ui_stage);
    }

    pub fn get_ui(&self) -> Vec<i32> {
        self.ui_stage.clone()
    }

    fn init(&self) -> std::io::Result<String> {
        std::fs::read_to_string("./profile")
    }
}
