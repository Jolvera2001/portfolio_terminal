use crossterm::event::KeyEvent;
use ratatui::widgets::Widget;

use crate::comms::{Command, Msg};

pub struct ContactScreen {
    
}

impl Widget for &ContactScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}


impl ContactScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            _ => Command::none(),
        }
    }
}