
use rand::Rng;

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
    selected_cell_index: usize
}

impl Board {
    pub fn new(width: i16, height: i16) -> Self {
        Self {
            cells: Vec::new(),
            board_width: width,
            board_height: height,
            selected_cell_index: 0
        }
    }

    pub fn initiate_board(&mut self, width: i16, height: i16){
        if width == 0 || height == 0{
            return;
        }

        self.create_cells((width * height) as usize);
        self.add_bombs(5);
    }

    fn create_cells(&mut self, cell_count: usize){
        for _i in 0..cell_count {
                self.cells.push(Cell::new(16, 16));
        }
        self.cells[self.selected_cell_index].set_is_selected(true);
    }

    fn add_bombs(&mut self, bomb_count: i16){
        let mut rng = rand::thread_rng();
        for _i in 0..bomb_count {
            let index = rng.gen_range(0..self.cells.len());
            self.cells[index].set_is_bomb(true);
        }
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
            InputEvent::Navigation(Direction::Up)  => {
                self.set_active_cell(self.selected_cell_index as i16 - self.board_width);
            },
            InputEvent::Navigation(Direction::Down)  => {
                self.set_active_cell(self.selected_cell_index as i16 + self.board_width);
            },
            InputEvent::Navigation(Direction::Left)  => {
                self.set_active_cell(self.selected_cell_index as i16 - 1);
            },
            InputEvent::Navigation(Direction::Right)  => {
                self.set_active_cell(self.selected_cell_index as i16 + 1);
            },
            _ => { },
        }
    }

    fn set_active_cell(&mut self, index: i16){
        if index < 0 || index >= self.cells.len() as i16 {
            return;
        }

        self.cells[self.selected_cell_index].set_is_selected(false);
        self.cells[index as usize].set_is_selected(true);
        self.selected_cell_index = index as usize;
    }

    pub fn select_active_cell(&mut self) {
        self.cells[self.selected_cell_index].select();
    }
}
