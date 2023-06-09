use std::{io, thread, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::{io::Stdout, ui::Draw};

use crate::{
    ui::{Screen},
    cell::{Cell},
};

pub struct App{
    // blocks: Vec<Box<dyn Cell>>
    cells: Vec<Box<Cell>>,
    board_width: i16,
    board_height: i16,
}

impl App {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            board_width: 9,
            board_height: 9,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        // Init stuff for rendering
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("stdout expect");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("terminal expect");

        // let mut screen: Screen<CrosstermBackend<Stdout>> = Screen::new();
        let screen: Screen = Screen::new();

        // Init game grid cells
        self.init_cells(10, 10);

        // Game loop
        loop {
            screen.draw_ui(&mut terminal, &self, self.board_width, self.board_height).expect("draw ui expect");
        
            thread::sleep(Duration::from_millis(5000));
            break;
        }
    
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn get_cells(&self) -> &Vec<Box<Cell>> {
        &self.cells
    }

    fn init_cells(&mut self, rows: i16, columns: i16){
        // Create cells
        for _i in 0..columns {
            for _j in 0..rows {
                self.cells.push(Box::new(Cell::new(16, 16)));
            }
        }
    }
}