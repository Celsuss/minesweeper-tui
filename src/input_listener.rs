use std::{
    sync::mpsc,
    time::{Instant, Duration},
};

use crossterm::{
    event::{self, Event, KeyEvent, KeyCode},
};

pub enum InputEvent {
    Input(KeyEvent),
    Tick,
    Quit
}

pub struct InputListener<'a> {
    rx:  &'a mpsc::Receiver<InputEvent>
}

impl<'a> InputListener<'a> {
    pub fn new(rx: &'a mpsc::Receiver<InputEvent>,) -> Self {
        InputListener { 
            rx
        }
    }

    pub fn quit(&self) -> bool {
        // Receive event from input thread
        match self.rx.recv().expect("rx recv expect") {
            InputEvent::Input(_) => { },
            InputEvent::Tick => { },
            InputEvent::Quit => return true,
        }

        false
    }

    pub fn handle_input(&self) {
        todo!();
    }
}

pub fn listen_for_input(tx: &mpsc::Sender<InputEvent>){
    let mut last_tick: Instant = Instant::now();
    let tick_rate: Duration = Duration::from_millis(200);
    
    loop {
        let timeout: Duration = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).expect("poll expect") {
            if let Event::Key(key) = event::read().expect("event read expect") {
                tx.send(InputEvent::Input(key)).expect("tx send expect");
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    
    }
}