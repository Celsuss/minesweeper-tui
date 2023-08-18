use std::{
    sync::mpsc,
    time::{Instant, Duration},
};

use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};

use crate::app::Difficulty;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq)]
pub enum InputEvent {
    Input(KeyEvent),
    Navigation(Direction),
    GameDifficulty(Difficulty),
    Select,
    Flag,
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
        let recv = self.rx.recv_timeout(Duration::from_millis(300));
        if recv.is_err(){
            return InputEvent::Tick;
        }

        match recv.unwrap() {
            InputEvent::Input(input) => match input {
                KeyEvent{ code: KeyCode::Char('d'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::Navigation(Direction::Right),
                KeyEvent{ code: KeyCode::Char('a'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::Navigation(Direction::Left),
                KeyEvent{ code: KeyCode::Char('w'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::Navigation(Direction::Up),
                KeyEvent{ code: KeyCode::Char('s'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::Navigation(Direction::Down),
                KeyEvent{ code: KeyCode::Char('e'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::GameDifficulty(Difficulty::Easy),
                KeyEvent{ code: KeyCode::Char('m'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::GameDifficulty(Difficulty::Medium),
                KeyEvent{ code: KeyCode::Char('h'), modifiers: KeyModifiers::NONE, ..} => return InputEvent::GameDifficulty(Difficulty::Hard),
                KeyEvent{ code: KeyCode::Enter, modifiers: KeyModifiers::NONE, ..} => return InputEvent::Select,
                _ => InputEvent::Input(input),
            },
            InputEvent::Flag => return InputEvent::Flag,
            InputEvent::Quit => return InputEvent::Quit,
            _ => return InputEvent::Tick,
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
                    KeyEvent{ code: KeyCode::Char('f'), modifiers: KeyModifiers::NONE, ..} => {
                        tx.send(InputEvent::Flag).expect("tx send expect");
                    },
                    _ => tx.send(InputEvent::Input(key)).expect("tx send expect"),
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
