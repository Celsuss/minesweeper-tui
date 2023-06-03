use crate::{
    ui::{Draw}
};

pub struct Block {
    width: u32,
    height: u32,
    value: i32,
    is_bomb: bool,
}

impl Draw for Block {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

impl Block {
    pub fn is_bomb(&self) -> bool{
        self.is_bomb
    }

    pub fn get_value(&self) -> i32{
        self.value
    }
}