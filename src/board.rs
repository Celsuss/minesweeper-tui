
use crate::{
    cell::Cell,
    input_listener::{
        InputEvent,
        Direction,
    },
};

pub struct Board{
    cells: Vec<Cell>,
    board_width: i16,
    board_height: i16,
}

impl Board {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            cells: Vec::new(),
            board_width: width,
            board_height: height,
        }
    }

    pub fn initiate_board(&mut self, width: i16, height: i16){
        // Create cells
        for _i in 0..width {
            for _j in 0..height {
                self.cells.push(Cell::new(16, 16));
            }
        }
        // TODO: Sett one cell as 'is_selected == true'
        self.cells[0].set_is_selected(true);
    }

    pub fn get_board_width(&self) -> i16{
        self.board_width
    }

    pub fn get_board_height(&self) -> i16{
        self.board_height
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn change_active_cell(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::Navigation(Direction::Up)  => { },
            InputEvent::Navigation(Direction::Down)  => { },
            InputEvent::Navigation(Direction::Left)  => { },
            InputEvent::Navigation(Direction::Right)  => { },
            _ => { },
        }
    }
}
