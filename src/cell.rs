use tui::{
    Frame,
    backend::{Backend},
    layout::{Rect, Layout, Direction, Constraint}
};

use crate::{
    ui::{Draw}
};

pub struct Cell {
    width: i16,
    height: i16,
    value: i32,
    is_bomb: bool,
    has_flag: bool,
}

impl<B: Backend> Draw<B> for Cell { 
    fn draw(&self, frame: &mut Frame<B>) {
        // code to actually draw a select box
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
    }
}

impl Cell {
    pub fn new(width: i16, height: i16) -> Self {
        Self { 
            width: (width), 
            height: (height), 
            value: (0), 
            is_bomb: (false),
            has_flag: (false)
        }
    }

    pub fn is_bomb(&self) -> bool{
        self.is_bomb
    }

    pub fn get_value(&self) -> i32{
        self.value
    }

    pub fn get_has_flag(&self) -> bool{
        self.has_flag
    }
}