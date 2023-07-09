use std::{io, thread, time::Duration, sync::mpsc::Receiver};
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

pub struct App{
    board: Board,
    score: i16,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::new(10, 10),
            score: 0,
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
        self.board.initiate_board(10, 10);

        // Game loop
        loop {
            screen.draw_ui(&mut terminal,
                           self,
                           &self.board).expect("Failed to draw ui");

            match input_listener.handle_input() {
                InputEvent::Navigation(direction) => self.board.change_active_cell(InputEvent::Navigation(direction)),
                InputEvent::Select => self.board.select_active_cell(),
                InputEvent::Quit => break,
                _  => { },
            }

            // if input_listener.handle_input() == InputEvent::Quit {
            //     break;
            // }
        }

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn get_score(&self) -> i16 {
        self.score
    }
}
