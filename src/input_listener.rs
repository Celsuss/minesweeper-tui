use std::{
    sync::mpsc,
    time::{Instant, Duration},
};

use crossterm::{
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers},
};

#[derive(PartialEq, Eq)]
pub enum InputEvent {
    Input(KeyEvent),
    Tick,
    Quit
}

pub struct InputListener<'a> {
    rx:  &'a mpsc::Receiver<InputEvent>,
}

impl<'a> InputListener<'a> {
    pub fn new(rx: &'a mpsc::Receiver<InputEvent>,) -> Self {
        InputListener {
            rx
        }
    }

    pub fn handle_input(&self) -> InputEvent {
        match self.rx.recv().expect("rx recv expect") {
            InputEvent::Input(input) => match input {
                // KeyEvent{ code: KeyCode::Char('x'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::Quit,
                _ => InputEvent::Input(input),
            }
            ,
            InputEvent::Tick => return InputEvent::Tick,
            InputEvent::Quit => return InputEvent::Quit,
        }
    }
}

pub fn listen_for_key_input(tx: &mpsc::Sender<InputEvent>){
    let mut last_tick: Instant = Instant::now();
    let tick_rate: Duration = Duration::from_millis(200);

    loop {
        let timeout: Duration = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).expect("poll expect") {
            if let Event::Key(key) = event::read().expect("event read expect") {
                match key {
                    KeyEvent{ code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE, ..} => {
                        tx.send(InputEvent::Quit).expect("tx send expect");
                    },
                    _ => tx.send(InputEvent::Input(key)).expect("tx send expect"),
                }

                tx.send(InputEvent::Input(key)).expect("tx send expect");
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
