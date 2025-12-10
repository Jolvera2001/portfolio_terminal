mod common;
mod portfolio;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app = portfolio::Portfolio::new();

    while app.running {
        terminal.draw(|f| app.view(f))?;

        app.update();

        // Handle events and map to a Message
        // let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        // while current_msg.is_some() {
        //     current_msg = update(&mut model, current_msg.unwrap());
        // }
    }

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
