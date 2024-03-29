use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    layout::Alignment,
    style::{Style, Color},
    text::Span
};

use crate::ui::Draw;

pub struct Cell {
    value: i16,
    is_open: bool,
    is_bomb: bool,
    is_flagged: bool,
    is_selected: bool,
}

impl<B: Backend> Draw<B> for Cell {
    fn draw(&self, frame: &mut Frame<B>, chunk: Rect, debug: bool) {
        let border_color = self.get_border_color(debug);

        // code to actually draw a select box
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        if self.is_open || self.is_flagged {
            let span = Span::styled(
                self.get_cell_text(),
                Style::default()
                    .fg(self.get_text_color())
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
    pub fn new() -> Self {
        Self {
            value: 0,
            is_open: false,
            is_bomb: false,
            is_flagged: false,
            is_selected: false
        }
    }

    pub fn is_bomb(&self) -> bool{
        self.is_bomb
    }

    pub fn set_is_bomb(&mut self, is_bomb: bool) {
        self.is_bomb = is_bomb;
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }

    pub fn toggle_is_flagged(&mut self) {
        if self.is_open {
            return
        }

        self.is_flagged = !self.is_flagged;
    }

    pub fn set_is_selected(&mut self, is_selected: bool){
        self.is_selected = is_selected;
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.is_flagged = false;
    }

    pub fn increment_value(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> i16 {
        self.value
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    fn get_cell_text(&self) -> String {
        if self.is_flagged == true {
            return "F".to_string();
        }
        else if self.is_bomb == true {
            return "B".to_string();
        }

        self.value.to_string()
    }

    fn get_border_color(&self, debug: bool) -> Color {
        if self.is_bomb && debug {
            return Color::Red;
        }
        else if self.is_selected {
            return Color::Cyan;
        }
        else if self.is_open == false {
            return Color::Gray;
        }

        self.get_text_color()
    }

    fn get_text_color(&self) -> Color {
        if self.is_flagged == true {
            return Color::Red;
        }
        else if self.is_bomb {
            return Color::Red;
        }

        match self.value {
            1 => return Color::Blue,
            2 => return Color::Yellow,
            3 => return Color::LightRed,
            4 => return Color::DarkGray,
            5 => return Color::Red,
            6 => return Color::Magenta,
            7 => return Color::Magenta,
            8 => return Color::Black,
            _ => return Color::White,
        }
    }
}
