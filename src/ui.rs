use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

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
    pub fn draw_components(&self, frame: &mut Frame<B>) {
        for component in self.components.iter() {
            // component.draw();
        }
    }

    pub fn update_chunks(&self){
        // Update all the chunks
    }
}