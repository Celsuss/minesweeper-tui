
use std::ptr::null;

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
        if width == 0 || height == 0 {
            return;
        }

        self.create_cells((width * height) as usize);
        self.add_bombs(10);
        self.update_cell_values();
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

    fn update_cell_values(&mut self) {
        for i in 0..self.cells.len() {
            let cell: &Cell = &self.cells[i];

            // Continue if this is not a bomb, if it is increase score of all adjacant cells
            if !cell.is_bomb() {
                continue;
            }

            // Convert index to x, y position
            let pos: (i16, i16) = self.get_pos_from_index(i as i16);
            for j in 0..3 {
                for k in 0..3 {
                    let neighbor_pos: (i16, i16) = (pos.0 + (k-1), pos.1 + (j-1));

                    let neighbor_index = self.get_index_from_pos(neighbor_pos.0, neighbor_pos.1);
                    if neighbor_index.is_some() == true {
                        self.cells[neighbor_index.unwrap()].increment_value();
                    }
                }
            }
        }
    }

    fn get_index_from_pos(&self, x: i16, y: i16) -> Option<usize> {
        if x < 0 || x >= self.board_width ||
            y < 0 || y >= self.board_height {
                return None;
            }

        Some((x + (y * self.board_width as i16)) as usize)
    }

    fn get_pos_from_index(&self, index: i16) -> (i16, i16) {
        let y = index / self.board_width;
        let x = index - (y * self.board_width);
        (x, y)
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
