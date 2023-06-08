use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
    Terminal,
};
use std::{error::Error, io};

pub struct Screen<B>
where
    B: Backend{
    pub components: Vec<Box<dyn Draw<B>>>,
    block_chunks: Vec<Layout>,
}

pub trait Draw<B: Backend>{
    fn draw(&self, frame: &mut Frame<B>);
}

impl<B: Backend> Screen<B>{
    pub fn new() -> Self{
        Self {
            components: Vec::new(),
            block_chunks: Vec::new()
        }
    }

    pub fn draw_ui(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Minesweeper")
                .borders(Borders::ALL);
            f.render_widget(block, size);

            self.draw_components(f);
        })?;

        Ok(())
    }

    fn draw_components(&self, frame: &mut Frame<B>) {
        let root_chunk = self.get_root_chunk(frame);

        for component in self.components.iter() {
            component.draw(frame);
        }
    }

    fn get_root_chunk(&self, frame: &mut Frame<B>) -> Vec<Rect>{
        let size: Rect = frame.size();
        let margin = 2;

        let root_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(margin)
            .constraints(
                [
                    Constraint::Length(30),   // Channels
                ]
                .as_ref(),
            )
            .split(size);
        
        root_chunk
    }

    pub fn update_chunks(&self){
        // Update all the chunks
    }
}