use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    layout::Alignment,
    style::{Style, Color},
    text::Span
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
        let border_color = if self.is_selected {
            Color::Green
        }
        else {
            Color::Gray
        };

        let background_color = if self.is_bomb {
            Color::Red
        }
        else {
            Color::Black
        };


        // code to actually draw a select box
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(background_color));

        if self.is_hidden == false {
            let span = Span::styled(
                self.value.to_string(),
                Style::default()
                    .fg(Color::Green)
            );

            let paragraph = Paragraph::new(span)
                .block(block)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            frame.render_widget(paragraph, chunk);
        }
        else {
            frame.render_widget(block, chunk);
        }
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

    pub fn set_is_bomb(&mut self, is_bomb: bool) {
        self.is_bomb = is_bomb;
    }

    pub fn get_value(&self) -> i32{
        self.value
    }

    pub fn get_has_flag(&self) -> bool{
        self.has_flag
    }

    pub fn set_has_flag(&mut self, has_flag: bool) {
        if self.is_hidden == false {
            return
        }

        self.has_flag = has_flag;
    }

    pub fn set_is_selected(&mut self, is_selected: bool){
        self.is_selected = is_selected;
    }

    pub fn select(&mut self) {
        self.is_hidden = false;
        self.has_flag = false;
    }
}
