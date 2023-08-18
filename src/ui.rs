use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap, Clear},
    style::{Style, Color},
    text::{Spans, Span, Text},
    Frame,
    Terminal,
};
use std::{
    io,
    time::Duration,
    collections::BTreeMap,

};

use crate::{
    app::App,
    board::Board,
};

pub struct Screen{
    cell_size: u16,
}

pub trait Draw<B: Backend>{
    fn draw(&self, frame: &mut Frame<B>, chunk: Rect);
}

impl Screen{
    pub fn new() -> Self{
        Self {
            cell_size: 3
        }
    }

    pub fn draw_ui<B: Backend>(&self, terminal: &mut Terminal<B>, app: &App, board: &Board, time: Duration) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Minesweeper")
                .borders(Borders::ALL);
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(board.get_board_height() as u16 * self.cell_size),
                        Constraint::Length(5),
                    ].as_ref())
                .margin(1)
                .split(f.size());

            self.draw_top_menu(f, board, time, chunks[0]);
            self.draw_board(f, chunks[1], board);
            self.draw_popup_windows(f, app, chunks[1]);
            self.draw_bottom_help_bar(f, chunks[2]);
        })?;

        Ok(())
    }

    fn draw_popup_windows<B: Backend>(&self, frame: &mut Frame<B>, app: &App, chunk: Rect) {
        if app.is_start_up() {
            self.draw_popup_window(frame, chunk, "Welcome".to_string()); 
        }
        else if app.get_is_game_over() {
            self.draw_popup_window(frame, chunk, "Game over".to_string()); 
        }
        else if app.get_is_victory() {
            self.draw_popup_window(frame, chunk, "Victory".to_string());
        }
    }

    fn draw_popup_window<B: Backend>(&self, frame: &mut Frame<B>, chunk: Rect, text_str: String) {
        let chunk = self.get_cell_center_chunk(chunk, 30, 6);
        let block = Block::default()
            .style(Style::default().fg(Color::Blue).bg(Color::Red))
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray));

        let text_style: Style = self.get_text_style();
        let mut text: Text = Text::styled(text_str, text_style);
        text.extend(self.get_restart_game_text());

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(Clear, chunk);
        frame.render_widget(paragraph, chunk);
    }

    fn get_restart_game_text(&self) -> Text{
        let key_bindings = BTreeMap::from([
            ("e", "Easy"),
            ("m", "Medium"),
            ("h", "Hard")
        ]);

        let text_style: Style = self.get_text_style();
        let mut text: Text = Text::default();

        for (key, description) in key_bindings.into_iter() {
            text.extend(
                Text::styled(
                    format!("{}: {}", key, description),
                    text_style))
        }
        text
    }

    fn draw_top_menu<B: Backend>(&self, frame: &mut Frame<B>, board: &Board, time: Duration, root_chunk: Rect){
        let mine_count = board.get_bomb_count();
        let flag_count = board.get_flag_count();
        let text_style = Style::default().fg(Color::Cyan);

        // Create the constraints
        let mut constraints = vec![];
        constraints.push(Constraint::Percentage(100));

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(root_chunk);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray));

        let span_vec = vec![
            Span::styled(
                format!("# mines: {}", mine_count as i16 - flag_count as i16),
                text_style
            ),
            Span::styled(
                " - ",
                text_style
            ),
            Span::styled(
                format!("Time: {}", time.as_secs().to_string()),
                text_style
            ),
        ];

        let spans = Spans::from(span_vec);
        let paragraph = Paragraph::new(spans)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, chunks[0]);
    }

    fn draw_bottom_help_bar<B: Backend>(&self, frame: &mut Frame<B>, chunk: Rect) {
        let key_bindings = BTreeMap::from([
            ("q", "Quit"),
            ("f", "Toggle flag"),
            ("Enter", "Select cell")
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray));
        let text_style: Style = self.get_text_style();
        let mut text: Text = Text::default();

        for (key, description) in key_bindings.into_iter() {
            text.extend(
                Text::styled(
                    format!("{}: {}", key, description),
                    text_style))
        }

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, chunk);
    }

    fn draw_board<B: Backend>(&self, frame: &mut Frame<B>, chunk: Rect, board: &Board) {
        // Create the vertical constraints
        let width = board.get_board_width() as u16 * self.cell_size;
        let height = board.get_board_height() as u16 * self.cell_size;
        let center_chunk = self.get_cell_center_chunk(chunk, width, height);
        self.draw_cells(frame, board, center_chunk);
    }

    fn get_cell_center_chunk(&self, chunk: Rect, width: u16, height: u16) -> Rect {
        let blank_width = (chunk.width - width) / 2;
        let blank_height = (chunk.height - height) / 2;

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(blank_height),
                Constraint::Length(height),
                Constraint::Max(blank_height)])
            .margin(0)
            .split(chunk);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Max(blank_width),
                Constraint::Length(width),
                Constraint::Max(blank_width)])
            .margin(0)
            .split(vertical_chunks[1])[1]
    }

    fn draw_cells<B: Backend>(&self, frame: &mut Frame<B>, board: &Board, root_chunk: Rect){
        let mut constraints = vec![];
        let mut i: usize = 0;
        while i < board.get_board_height() {
            constraints.push(Constraint::Length(self.cell_size));
            i += 1;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_ref())
            .margin(0)
            .split(root_chunk);

        let mut cell_index: usize = 0;
        for chunk in chunks{
            self.draw_horizontal_cells(frame, board, chunk, &mut cell_index);
        }
    }

    fn draw_horizontal_cells<B: Backend>(&self, frame: &mut Frame<B>, board: &Board, root_chunk: Rect, cell_index: &mut usize){
        let board_width = board.get_board_width();
        // Create the constraints
        let mut constraints = vec![];
        let mut i: usize = 0;
        while i < board_width {
            constraints.push(Constraint::Length(self.cell_size));
            i += 1;
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .margin(0)
            .split(root_chunk);

        for chunk in chunks {
            board.get_cells()[*cell_index].draw(frame, chunk);
            *cell_index += 1;
        }
    }

    fn get_text_style(&self) -> Style {
        Style::default().fg(Color::Cyan)
    }
}

