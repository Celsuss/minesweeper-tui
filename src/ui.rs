use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
    Terminal,
};
use std::{io};

use crate::app::App;

pub struct Screen{

}

pub trait Draw<B: Backend>{
    fn draw(&self, frame: &mut Frame<B>, chunk: Rect);
}

impl Screen{
    pub fn new() -> Self{
        Self {
            
        }
    }

    pub fn draw_ui<B: Backend>(&self, terminal: &mut Terminal<B>, app: &App, board_width: i16, board_height: i16) -> io::Result<()> {
        println!("test");
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Minesweeper")
                .borders(Borders::ALL);
            f.render_widget(block, size);

            self.draw_board(f, app, board_width, board_height);
        })?;

        Ok(())
    }

    fn draw_board<B: Backend>(&self, frame: &mut Frame<B>, app: &App, board_width: i16, board_height: i16) {
        // TODO: Fix this
        // Create the constraints
        let mut constraints = vec![];
        let mut i: i16 = 0;
        while i < board_width {
            constraints.push(Constraint::Percentage(100 / (board_width as u16)));
            i += 1;
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            // .constraints(constraints.as_ref())
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .margin(2)
            .split(frame.size());

        app.get_cells()[0].draw(frame, chunks[0]);
        app.get_cells()[0].draw(frame, chunks[1]);

        // i = 0;
        // for component in app.get_cells().iter() {
        //     component.draw(frame, chunks[0]);
        //     break;
        // }
    }
}