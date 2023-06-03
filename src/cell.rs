use crate::{
    ui::{Draw}
};

pub struct Cell {
    width: i16,
    height: i16,
    value: i32,
    is_bomb: bool,
    has_flag: bool,
}

impl Draw for Cell {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

impl Cell {
    pub fn new(width: i16, height: i16) -> Self {
        Self { 
            width: (width), 
            height: (height), 
            value: (0), 
            is_bomb: (false),
            has_flag: (false)
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
}