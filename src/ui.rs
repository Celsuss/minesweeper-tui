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
        // Create the vertical constraints
        let mut constraints = vec![];
        let mut i: i16 = 0;
        while i < board_height {
            constraints.push(Constraint::Percentage(100 / (board_height as u16)));
            i += 1;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_ref())
            .margin(2)
            .split(frame.size());

        let mut cell_index: usize = 0;

        for chunk in chunks{
            self.draw_horizontal_cells(frame, app, chunk, board_width, &mut cell_index);
        }
    }

    fn draw_horizontal_cells<B: Backend>(&self, frame: &mut Frame<B>, app: &App, root_chunk: Rect, board_width: i16, cell_index: &mut usize){
        // Create the constraints
        let mut constraints = vec![];
        let mut i: i16 = 0;
        while i < board_width {
            constraints.push(Constraint::Percentage(100 / (board_width as u16)));
            i += 1;
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(root_chunk);

        for chunk in chunks {
            app.get_cells()[*cell_index].draw(frame, chunk);
            *cell_index += 1;
        }
    }
}
