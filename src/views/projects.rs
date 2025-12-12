use crossterm::event::KeyEvent;
use ratatui::widgets::Widget;

use crate::comms::{Command, Msg};

pub struct ProjectsScreen {
    
}

impl Widget for &ProjectsScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}

impl ProjectsScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            _ => Command::none(),
        }
    }
}