
use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
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
            let mut a = PathBuf::from(line.trim().to_owned());
            a.push(PathBuf::from("\u{005C}"));
            home.push(a);
        }
        home.pop();
        home.remove(0);
    }

        let result = Profile {
            ui_stage: vec![0, 0, 1, 0],
            tiles: home.clone(),
        };
        println!("{:?}", home);
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

    }

    pub fn print_ui(&self) {
        println!("{:?}", self.ui_stage);
    }

    pub fn get_ui(&self) -> Vec<i32> {
        self.ui_stage.clone()
    }

    fn init(&self) -> std::io::Result<String> {
        let str = fs::read_to_string("profile/profile.json")?;
        let obj = from_str(str.as_str())?;
        Ok(obj)
    }

    pub fn save(&self) {
        let ser = serde_json::to_string(self).expect("序列化失败");
        fs::create_dir_all("../profile").expect("profile文件夹创建失败");

        let file_path = Path::new("../profile").join("profile.json");
        let mut file = File::create(file_path).expect("profile文件创建成功");
        writeln!(file, "{}", ser).expect("写入序列化数据失败");
    }

    pub fn init_tiles(&self) -> Option<Vec<PathBuf>> {
        match self.tiles.is_empty() {
            true => {
                None
            }
            false => {
                Some(self.tiles.clone())
            }
        }
    }
    pub fn add_tiles(&mut self, path: PathBuf) {
        self.tiles.push(path);
    }
    pub fn remove_tiles(&mut self, target: usize) {
        self.tiles.remove(target);
    }

}

impl Drop for Profile {
    fn drop(&mut self) {
        self.save();
    }
}
