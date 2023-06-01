use gui::Draw;

struct Block {
    width: u32,
    height: u32,
    value: i32,
    is_bomb: bool,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}