use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

mod app;
mod ui;
mod cell;
mod input_listener;
mod board;

fn main() -> Result<(), io::Error> {
    enable_raw_mode().expect("Enable raw mode expect");

    let (tx, rx): (Sender<input_listener::InputEvent>, Receiver<input_listener::InputEvent>) = mpsc::channel();
    thread::spawn(move || input_listener::listen_for_input(&tx));

    let mut app: app::App = app::App::new();
    app.run(&rx).expect("Run expected");

    disable_raw_mode()?;

    Ok(())
}
