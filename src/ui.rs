use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Style, Color, Modifier},
    text::{Spans, Span, Text},
    Frame,
    Terminal,
};
use std::{
    io,
    time::Duration,
    collections::{HashMap, BTreeMap},

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

    pub fn draw_ui<B: Backend>(&self, terminal: &mut Terminal<B>, app: &App, board: &Board, time: Duration, game_over: bool) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Minesweeper")
                .borders(Borders::ALL);
            f.render_widget(block, size);

            // TODO: Draw top menu and board
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(5),
                        Constraint::Length(5),
                    ].as_ref())
                .margin(1)
                .split(f.size());

            self.draw_top_menu(f, app, board, time, chunks[0]);
            self.draw_board(f, app, chunks[1], board);
            self.draw_bottom_help_bar(f, chunks[2]);
        })?;

        Ok(())
    }

    fn draw_top_menu<B: Backend>(&self, frame: &mut Frame<B>, app: &App, board: &Board, time: Duration, root_chunk: Rect){
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

        let mut span_vec = vec![
            Span::styled(
                format!("# mines: {}", mine_count as i16 - flag_count as i16),
                text_style
            ),
            Span::styled(
                " - ",
                text_style
            ),
        ];

        if app.get_game_over() {
            span_vec.push(
                Span::styled(
                    "Game Over",
                    text_style
                )
            );
            span_vec.push(
                Span::styled(
                    " - ",
                    text_style
                )
            );
        }

        span_vec.push(
            Span::styled(
                format!("Time: {}", time.as_secs().to_string()),
                text_style
            ),
        );

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


        // let mut span_array: Vec<Span> = Vec::default();
        // for (key, description) in key_bindings {
        //     let span = Span::styled(
        //                  format!("{}: {}", key, description),
        //                  Style::default()
        //                    .fg(Color::Blue));
        //     span_array.push(span);

        //     span_array.push(
        //         Span::styled(
        //             " \n",
        //             Style::default())
        //     );
        // }

        // let spans = Spans::from(span_array);
        // let paragraph = Paragraph::new(spans)
        //     .block(block)
        //     .alignment(Alignment::Center)
        //     .wrap(Wrap { trim: true });

        // frame.render_widget(paragraph, chunk);
    }

    fn draw_board<B: Backend>(&self, frame: &mut Frame<B>, app: &App, chunk: Rect, board: &Board) {
        // Create the vertical constraints
        let center_chunk = self.get_cell_center_chunk(chunk, board);
        self.draw_cells(frame, board, center_chunk);
    }

    fn get_cell_center_chunk(&self, chunk: Rect, board: &Board) -> Rect {
        let cells_width = board.get_board_width() as u16 * self.cell_size;
        let cells_height = board.get_board_height() as u16 * self.cell_size;

        let blank_width = (chunk.width - cells_width) / 2;
        let blank_height = (chunk.height - cells_height) / 2;


        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(blank_height),
                Constraint::Length(cells_height),
                Constraint::Max(blank_height)])
            .margin(0)
            .split(chunk);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Max(blank_width),
                Constraint::Length(cells_width),
                Constraint::Max(blank_width)])
            .margin(0)
            .split(vertical_chunks[1])[1]
    }

    fn draw_cells<B: Backend>(&self, frame: &mut Frame<B>, board: &Board, root_chunk: Rect){
        let mut constraints = vec![];
        let mut i: usize = 0;
        while i < board.get_board_height() {
            // constraints.push(Constraint::Percentage(100 / (board.get_board_height() as u16)));
            //constraints.push(Constraint::Percentage(10));
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
            // constraints.push(Constraint::Percentage(100 / (board_width as u16)));
            // constraints.push(Constraint::Percentage(10));
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

