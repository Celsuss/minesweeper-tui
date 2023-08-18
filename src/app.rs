use std::{
    io,
    time::{Duration, Instant},
    sync::mpsc::Receiver
};
use tui::{
    backend::{CrosstermBackend},
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    ui::Screen,
    input_listener::{InputEvent, InputListener},
    board::Board,
};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct App{
    board: Board,
    start_time: Instant,
    end_time: Instant,
    game_over: bool,
    victory: bool,
    quit: bool,
    start_up: bool,
    change_difficulty: bool,
    difficulty: Difficulty,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            start_time: Instant::now(),
            end_time: Instant::now(),
            game_over: false,
            victory: false,
            quit: false,
            start_up: true,
            change_difficulty: true,
            difficulty: Difficulty::Easy,
        }
    }

    pub fn run(&mut self, rx: &Receiver<InputEvent>) -> Result<(), Box<dyn std::error::Error>>{
        // Init stuff for rendering
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("stdout expect");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("terminal expect");

        let screen: Screen = Screen::new();
        let input_listener: InputListener = InputListener::new(rx);

        let mut game_duration: Duration = Instant::now() - self.start_time;

        // Game loop
        while !self.quit {
            if !self.game_over {
                game_duration = Instant::now() - self.start_time;
            }

            screen.draw_ui(&mut terminal,
                           self,
                           &self.board,
                           game_duration).expect("Failed to draw ui");

            self.handle_input(&input_listener);
        }

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn handle_input(&mut self, input_listener: &InputListener) {
        match input_listener.handle_input() {
            InputEvent::Navigation(direction) => {
                if !self.game_over {
                    self.board.change_active_cell(InputEvent::Navigation(direction))
                }
            },
            InputEvent::Select => {
                if !self.game_over {
                    self.board.select_active_cell();
                    if self.board.is_selected_cell_bomb() {
                        self.set_is_game_over(true);
                    }
                    else if self.board.is_all_safe_cells_open() {
                        self.set_is_victory(true);
                    }
                }
            },
            InputEvent::GameDifficulty(difficulty) => {
                if self.change_difficulty {
                    self.initiate_game(difficulty);
                }
            }
            InputEvent::Flag => self.board.toggle_active_cell_flag(),
            InputEvent::Quit => self.quit = true,
            _  => { },
        }
    }

    fn initiate_game(&mut self, difficulty: Difficulty){
        self.difficulty = difficulty;
        self.game_over = false;
        self.victory = false;
        self.change_difficulty = false;
        self.start_up = false;

        self.board.initiate_board(self.difficulty);
        self.start_time = Instant::now();
    }

    fn end_game(&mut self) {
        self.end_time = Instant::now();
        self.change_difficulty = true;
    }

    fn set_is_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
        if self.game_over {
            self.end_game();
        }
    }

    pub fn get_is_game_over(&self) -> bool {
        self.game_over
    }

    fn set_is_victory(&mut self, victory: bool) {
        self.victory = victory;
        if self.victory {
            self.end_game();
        }
    }

    pub fn get_is_victory(&self) -> bool {
        self.victory
    }

    pub fn is_start_up(&self) -> bool {
        self.start_up
    }
}
