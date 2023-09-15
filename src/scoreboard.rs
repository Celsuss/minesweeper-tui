use home;

pub struct Scoreboard {
    scoreboard_path: String
}

impl Scoreboard {
    pub fn new() -> Self {
        Self {
            scoreboard_path: "/minesweeper-tui/scoreboard.txt".to_string(),
        }
    }

    pub fn save_score(&self) {
        let home_path: String = self.get_home_path();
        if home_path == "" {
            return
        }


    }

    pub fn get_score(&self) {
        let home_path: String = self.get_home_path();
        if home_path == "" {
            return
        }


    }

    fn get_scoreboard_path(&self) -> String {
        let home_path: String = self.get_home_path();
        if home_path == "" {
            return "".to_string()
        }

        "".to_string()
    }

    fn get_home_path(&self) -> String {
        match home::home_dir() {
            Some(path) => return path.display().to_string(),
            None => return "".to_string(),
        }
    }
}
