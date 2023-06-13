use std::{
    sync::mpsc,
};

pub enum InputEvent {
    Tick,
    Quit
}

pub struct InputListener {

}

impl InputListener {

}

pub fn listen_for_input(tx: &mpsc::Sender<InputEvent>){

}