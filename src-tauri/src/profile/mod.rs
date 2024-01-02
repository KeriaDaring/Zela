
use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::process::Command;
#[derive(Debug, Deserialize, Serialize)]
use std::env;
pub struct Profile {
    name: String,
    ui_stage: Vec<i32>,
    tiles: Vec<PathBuf>,
}


impl Profile {
    pub fn new() -> Self {
        
        let mut home: Vec<PathBuf> = vec![];

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

        let result = Profile {
            name: String::from("profile"),
            ui_stage: vec![0, 1, 1, 0],
            tiles: home,
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
