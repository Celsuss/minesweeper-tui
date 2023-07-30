use std::{
    io,
    thread,
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
    score: i16,
    start_time: Instant,
    game_over: bool,
    victory: bool,
    difficulty: Difficulty,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            score: 0,
            start_time: Instant::now(),
            game_over: false,
            victory: false,
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

        // Init game grid cells
        self.initiate_game();

        // Game loop
        loop {
            let game_duration: Duration = Instant::now() - self.start_time;
            screen.draw_ui(&mut terminal,
                           self,
                           &self.board,
                           game_duration,
                           self.game_over).expect("Failed to draw ui");

            match input_listener.handle_input() {
                InputEvent::Navigation(direction) => {
                    if !self.game_over {
                        self.board.change_active_cell(InputEvent::Navigation(direction))
                    }
                },
                InputEvent::Select => {
                    if !self.game_over {
                        self.board.select_active_cell(&mut self.game_over)
                    }
                },
                InputEvent::Flag => self.board.toggle_active_cell_flag(),
                InputEvent::Quit => break,
                _  => { },
            }
        }

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn initiate_game(&mut self){
        self.board.initiate_board(self.difficulty);
        self.start_time = Instant::now();
    }

    pub fn get_score(&self) -> i16 {
        self.score
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over
    }
}
