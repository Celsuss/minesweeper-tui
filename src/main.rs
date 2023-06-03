use crossterm::{
    // event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

mod app;
mod ui;
mod block;

fn main() -> Result<(), io::Error> {
    enable_raw_mode().expect("Enable raw mode expect");

    let mut app: app::App = app::App {  };
    app.run().expect("Run expected");

    disable_raw_mode()?;

    Ok(())
}
