use home;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Scores {
    scores: Vec<i16>,
    names: Vec<String>,
}

pub struct Scoreboard {
    scoreboard_path: String,
}

impl Scoreboard {
    pub fn new() -> Self {
        Self {
            scoreboard_path: "/minesweeper-tui/scoreboard.txt".to_string(),
        }
    }

    pub fn save_score(&self) -> Result<()> {
        let file_path: String = self.get_scoreboard_path();
        if file_path == "" {
            return Ok(());
        }

        Ok(())
    }

    pub fn get_score(&self) {
        let file_path: String = self.get_scoreboard_path();
        if file_path == "" {
            return;
        }
    }

    fn load_scores(&self) -> Result<()> {
        let file_path: String = self.get_scoreboard_path();
        if file_path == "" {
            return Ok(());
        }

        let _file = File::open(file_path);

        Ok(())
    }

    fn get_scoreboard_path(&self) -> String {
        let home_path: String = self.get_home_path();
        if home_path == "" {
            return "".to_string();
        }

        format!("{}{}", home_path, self.scoreboard_path).to_string()
    }

    fn get_home_path(&self) -> String {
        match home::home_dir() {
            Some(path) => return path.display().to_string(),
            None => return "".to_string(),
        }
    }
}
