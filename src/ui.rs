use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

pub struct Screen{
    pub components: Vec<Box<dyn Draw>>,
    block_chunks: Vec<layout>,
}

impl Screen {
    pub fn draw_components(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn draw_ui(&self, f: &mut Frame<B>) {

    }

    pub fn update_chunks(&self){
        
    }
}