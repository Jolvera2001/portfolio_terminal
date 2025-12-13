use crossterm::event::KeyEvent;
use ratatui::{text::{Line, Text}, widgets::{Paragraph, Widget}};

use crate::comms::{Command, Msg};

pub struct ProjectsScreen {
    
}

impl Widget for &ProjectsScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let intro_text = Text::from(vec![Line::from(vec!["Hello World!".into()])]);
        Paragraph::new(intro_text).render(area, buf);
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