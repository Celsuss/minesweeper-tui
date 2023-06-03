use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
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

use crate::ui;
use crate::block;

pub struct App{
    // blocks: Vec<Box<dyn Block>>,
}

impl App {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("stdout expect");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("terminal expect");
        self.init_blocks();

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .title("Block")
                    .borders(Borders::ALL);
                f.render_widget(block, size);
            })?;
        
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

    fn draw(&self){

    }

    fn init_blocks(&self){
        let rows: i16 = 8;
        let columns: i16 = 8;

    }
}