use crossterm::{
    // event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

mod app;

fn main() -> Result<(), io::Error> {
    enable_raw_mode().expect("Enable raw mode expect");

    app::start_app().expect("start app ");

    // restore terminal
    disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    Ok(())
}
