
use std::collections::HashMap;
use rand::Rng;

use crate::{
    cell::Cell,
    input_listener::{
        InputEvent,
        Direction,
    },
    app::Difficulty,
};

pub struct Board{
    cells: Vec<Cell>,
    board_width: usize,
    board_height: usize,
    selected_cell_index: usize,
    bomb_count: usize,
    flag_count: usize,
    board_size_map: HashMap<Difficulty, (usize, usize)>,
    board_bombs_map: HashMap<Difficulty, usize>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            board_width: 0,
            board_height: 0,
            selected_cell_index: 0,
            bomb_count: 0,
            flag_count: 0,
            board_size_map: HashMap::from([
                (Difficulty::Easy, (9, 9)),
                (Difficulty::Medium, (16, 16)),
                (Difficulty::Hard, (30, 16)),
            ]),
            board_bombs_map: HashMap::from([
                (Difficulty::Easy, 10),
                (Difficulty::Medium, 32),
                (Difficulty::Hard, 60),
            ])
        }
    }

    pub fn initiate_board(&mut self, difficulty: Difficulty){
        self.board_width = self.board_size_map[&difficulty].0;
        self.board_height = self.board_size_map[&difficulty].1;
        self.bomb_count = self.board_bombs_map[&difficulty];

        self.create_cells((self.board_width * self.board_height) as usize);
        self.add_bombs(self.bomb_count as i16);
        self.update_cell_values();
    }

    fn create_cells(&mut self, cell_count: usize){
        for _i in 0..cell_count {
                self.cells.push(Cell::new());
        }
        self.cells[self.selected_cell_index].set_is_selected(true);
    }

    fn add_bombs(&mut self, bomb_count: i16){
        self.bomb_count = bomb_count as usize;
        let mut rng = rand::thread_rng();
        for _i in 0..bomb_count {
            let mut index = rng.gen_range(0..self.cells.len());
            while self.cells[index].is_bomb() {
                index = rng.gen_range(0..self.cells.len());
            }
            self.cells[index].set_is_bomb(true);
        }
    }

    fn update_cell_values(&mut self) {
        for i in 0..self.cells.len() {
            let cell: &Cell = &self.cells[i];

            // Continue if this is not a bomb, if it is increase score of all adjacent cells
            if !cell.is_bomb() {
                continue;
            }

            // Iterate over the neighbors and increment their values
            let neighbors_indexes: Vec<usize> = self.get_cell_neighbors_indexes(i as i16);
            for i in neighbors_indexes {
                self.cells[i].increment_value();
            }
        }
    }

    fn get_index_from_pos(&self, x: i16, y: i16) -> Option<usize> {
        if x < 0 || x >= self.board_width as i16 ||
            y < 0 || y >= self.board_height as i16 {
                return None;
            }

        Some((x + (y * self.board_width as i16)) as usize)
    }

    fn get_pos_from_index(&self, index: i16) -> (i16, i16) {
        let y = index / (self.board_width as i16);
        let x = index - (y * (self.board_width as i16));
        (x, y)
    }

    pub fn get_board_width(&self) -> usize {
        self.board_width
    }

    pub fn get_board_height(&self) -> usize {
        self.board_height
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn change_active_cell(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::Navigation(Direction::Up)  => {
                self.set_active_cell(self.selected_cell_index as i16 - self.board_width as i16);
            },
            InputEvent::Navigation(Direction::Down)  => {
                self.set_active_cell(self.selected_cell_index as i16 + self.board_width as i16);
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

    pub fn toggle_active_cell_flag(&mut self) {
        self.cells[self.selected_cell_index].toggle_is_flagged();
        if self.cells[self.selected_cell_index].is_flagged() {
            self.flag_count += 1;
        }
        else {
            self.flag_count -= 1;
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

    pub fn select_active_cell(&mut self, game_over: &mut bool) {
        self.cells[self.selected_cell_index].select();
        if self.cells[self.selected_cell_index].is_bomb() {
            *game_over = true;
        }
    }

    pub fn get_bomb_count(&self) -> usize {
        self.bomb_count
    }

    pub fn get_flag_count(&self) -> usize {
        self.flag_count
    }

    fn get_cell_neighbors_indexes(&self, index: i16) -> Vec<usize> {
        let mut neighbors = vec![];
        let pos: (i16, i16) = self.get_pos_from_index(index as i16);
        for j in 0..3 {
            for k in 0..3 {
                let neighbor_pos: (i16, i16) = (pos.0 + (k-1), pos.1 + (j-1));
                let neighbor_index = self.get_index_from_pos(neighbor_pos.0, neighbor_pos.1);
                if neighbor_index.is_some() == true && neighbor_index.unwrap() != index as usize {
                    neighbors.push(neighbor_index.unwrap());
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_sizes() {
        let mut board: Board = Board::new();
        board.initiate_board(Difficulty::Easy);
        assert_eq!(board.get_board_height(), 9);
        assert_eq!(board.get_board_width(), 9);

        board.initiate_board(Difficulty::Medium);
        assert_eq!(board.get_board_height(), 16);
        assert_eq!(board.get_board_width(), 16);

        board.initiate_board(Difficulty::Hard);
        assert_eq!(board.get_board_height(), 16);
        assert_eq!(board.get_board_width(), 30);
    }

    #[test]
    fn test_board_bomb_count() {
        let mut board: Board = Board::new();
        board.initiate_board(Difficulty::Easy);
        assert_eq!(board.get_bomb_count(), 10);

        board.initiate_board(Difficulty::Medium);
        assert_eq!(board.get_bomb_count(), 32);

        board.initiate_board(Difficulty::Hard);
        assert_eq!(board.get_bomb_count(), 60);
    }

    #[test]
    fn test_change_active_cell(){
        let mut board: Board = Board::new();

        // Test change active cell without initiating the board
        board.change_active_cell(InputEvent::Navigation(Direction::Up));

        // Test increment active cell
        board.initiate_board(Difficulty::Easy);
        assert_eq!(board.selected_cell_index, 0);
        board.change_active_cell(InputEvent::Navigation(Direction::Right));
        assert_eq!(board.selected_cell_index, 1);

        // Test decrease active cell back
        board.change_active_cell(InputEvent::Navigation(Direction::Left));
        assert_eq!(board.selected_cell_index, 0);

        // Test go down one row
        board.change_active_cell(InputEvent::Navigation(Direction::Down));
        assert_eq!(board.selected_cell_index, board.board_width);

        // Test go back up one row
        board.change_active_cell(InputEvent::Navigation(Direction::Up));
        assert_eq!(board.selected_cell_index, 0);

        // Test decrease from 0
        board.change_active_cell(InputEvent::Navigation(Direction::Left));
        assert_eq!(board.selected_cell_index, 0);

        // Test go up one row from first row
        board.change_active_cell(InputEvent::Navigation(Direction::Up));
        assert_eq!(board.selected_cell_index, 0);
    }

    #[test]
    fn test_get_pos_from_index(){
        let mut board: Board = Board::new();
        board.initiate_board(Difficulty::Easy);

        assert_eq!(board.get_pos_from_index(0), (0, 0));
        assert_eq!(board.get_pos_from_index(1), (1, 0));
        assert_eq!(board.get_pos_from_index(board.board_width as i16), (0, 1));
        // TODO: Write more tests
    }

    #[test]
    fn test_get_index_from_pos(){
        let mut board: Board = Board::new();
        board.initiate_board(Difficulty::Easy);

        assert_eq!(board.get_index_from_pos(0, 0).unwrap(), 0);
        assert_eq!(board.get_index_from_pos(1, 0).unwrap(), 1);
        assert_eq!(board.get_index_from_pos(0, 1).unwrap(), board.board_width);
        // TODO: Write more tests
    }
}
