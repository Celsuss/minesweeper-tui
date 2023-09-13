use std::env;

pub struct Scoreboard {
    
}

impl Scoreboard {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn save_score() {
        
    }

    fn get_home_path() -> String {
        // TODO Change from deprecated function to a crate
        // match env::home_dir() {
        //     Some(path) => return path.display().to_string(),
        //     None => return "".to_string()
        // }
        "".to_string()
    }
}
