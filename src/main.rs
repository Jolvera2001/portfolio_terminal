mod common;
mod comms;
mod portfolio;
mod views;

use std::time::Duration;

use crossterm::event::{self, Event};
use tokio::sync::mpsc;

use crate::{comms::Msg, portfolio::GlobalMsg};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app = portfolio::Portfolio::new();

    let (tx, mut rx) = mpsc::unbounded_channel();

    let event_handle = {
        let event_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Ok(Event::Key(key)) = event::read() {
                        if key.kind == event::KeyEventKind::Press {
                            if event_tx.send(Msg::Global(GlobalMsg::KeyPress(key))).is_err() {
                                break;
                            }
                        }
                    }
                }
            }
        })
    };

    let mut should_quit = false;

    // main loop
    while !should_quit {
        terminal.draw(|f| app.view(f))?;

        if let Some(msg) = rx.recv().await {
            if matches!(msg, Msg::Global(GlobalMsg::Quit)) {
                should_quit = true;
                continue;
            }

            let command = app.update(msg);

            command.execute(tx.clone());
        }
    }

    drop(tx);
    drop(rx);
    event_handle.abort();
    let _ = event_handle.await;

    tui::restore_terminal()?;
    Ok(())
}

mod tui {
    use ratatui::{
        Terminal,
        backend::{Backend, CrosstermBackend},
        crossterm::{
            ExecutableCommand,
            terminal::{
                EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
            },
        },
    };
    use std::{io::stdout, panic};

    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
