use std::{io, thread, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::io::Stdout;

use crate::{
    ui::{Screen},
    cell::{Cell},
};

pub struct App{
    // blocks: Vec<Box<dyn Cell>>
    cells: Vec<Box<Cell>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            cells: Vec::new()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        // Init stuff for rendering
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("stdout expect");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("terminal expect");

        let screen: Screen<CrosstermBackend<Stdout>> = Screen::new();

        // Init game grid cells
        self.init_cells(12, 12);

        // Game loop
        loop {
            screen.draw_ui(&mut terminal)?;
        
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

    fn init_cells(&mut self, rows: i16, columns: i16){
        // Create cells
        for i in 0..columns {
            for j in 0..rows {
                self.cells.push(Box::new(Cell::new(16, 16)));
            }
        }
    }
}