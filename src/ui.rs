use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Style, Color, Modifier},
    text::{Spans, Span},
    Frame,
    Terminal,
};
use std::{
    io,
    time::Duration,
};

use crate::{
    app::App,
    board::Board,
};

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

    pub fn draw_ui<B: Backend>(&self, terminal: &mut Terminal<B>, app: &App, board: &Board, time: Duration) -> io::Result<()> {
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
                    [Constraint::Percentage(10),
                     Constraint::Percentage(90)].as_ref())
                .margin(1)
                .split(f.size());

            self.draw_top_menu(f, app, time, chunks[0]);
            self.draw_board(f, app, chunks[1], board);
        })?;

        Ok(())
    }

    fn draw_top_menu<B: Backend>(&self, frame: &mut Frame<B>, app: &App, time: Duration, root_chunk: Rect){
        // TODO: Draw score, timer and maybe more

        let score = app.get_score();

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

        let spans = Spans::from(vec![
            Span::styled(
                format!("Score: {}", score),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            ),
            Span::raw(" "),
            Span::styled(
                format!("Time: {}", time.as_secs().to_string()),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            ),
        ]);

        // let span = Span::styled(
        //     "Score: {score}",
        //     Style::default()
        //         .fg(Color::Yellow)
        //         .add_modifier(Modifier::BOLD)
        // );

        let paragraph = Paragraph::new(spans)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, chunks[0]);
    }

    fn draw_board<B: Backend>(&self, frame: &mut Frame<B>, app: &App, root_chunk: Rect, board: &Board) {
        // Create the vertical constraints
        self.draw_cells(frame, board, root_chunk);
    }

    fn draw_cells<B: Backend>(&self, frame: &mut Frame<B>, board: &Board, root_chunk: Rect){
        let mut constraints = vec![];
        let mut i: i16 = 0;
        while i < board.get_board_height() {
            // constraints.push(Constraint::Percentage(100 / (board.get_board_height() as u16)));
            constraints.push(Constraint::Percentage(10));
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
        let mut i: i16 = 0;
        while i < board_width {
            // constraints.push(Constraint::Percentage(100 / (board_width as u16)));
            constraints.push(Constraint::Percentage(10));
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
}

