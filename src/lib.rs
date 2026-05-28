use std::io;

use ratatui::{Frame, Terminal, backend::CrosstermBackend};

use crate::{error::Result, tui::state::State};

pub mod error;
pub mod git;
pub mod metrics;
pub mod tui;

pub fn run() -> Result<()> {
    tui();
    todo!("")
}

pub fn tui() -> Result<()> {
    let mut state = State::new();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| render(frame))?;
    }

    Ok(())
}

pub fn render(frame: &mut Frame) {
    todo!("")
}
