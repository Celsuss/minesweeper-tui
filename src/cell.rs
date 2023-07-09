use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    style::{Style, Color}
};

use crate::{
    ui::Draw
};

pub struct Cell {
    width: i16,
    height: i16,
    value: i32,
    is_hidden: bool,
    is_bomb: bool,
    has_flag: bool,
    is_selected: bool,
}

impl<B: Backend> Draw<B> for Cell {
    fn draw(&self, frame: &mut Frame<B>, chunk: Rect) {
        let mut color = Color::Gray;
        if self.is_selected {
            color = Color::Green;
        }

        // code to actually draw a select box
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color));
            //.style(Style::default().bg(Color::Blue));
        frame.render_widget(block, chunk);
        // TODO: Add assertions so block is not of no size.
    }
}

impl Cell {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            width: (width),
            height: (height),
            value: 0,
            is_hidden: true,
            is_bomb: false,
            has_flag: false,
            is_selected: false
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

    pub fn set_is_selected(&mut self, is_selected: bool){
        self.is_selected = is_selected;
    }

    pub fn select(&mut self) {
        
    }
}
