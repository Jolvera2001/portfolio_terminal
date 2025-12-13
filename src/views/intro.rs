use std::collections::HashMap;

use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::Paragraph};
use serde::{Deserialize, Serialize};

use crate::comms::{Command, Msg};

#[derive(Serialize, Deserialize, Debug)]
pub struct IntroScreenContent {
    sections: HashMap<String, String>,
    order: Vec<String>,
}

pub struct IntroScreen {
    cursor: usize,
    content: IntroScreenContent,
}

impl Widget for &IntroScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let key = self.content.order.get(self.cursor);

        let intro_text = self
            .content
            .order
            .get(self.cursor)
            .and_then(|k| self.content.sections.get(k))
            .map(|text| Text::from(text.as_str()))
            .unwrap_or_else(|| Text::from(""));

        Paragraph::new(intro_text).render(area, buf);
    }
}

impl IntroScreen {
    pub fn new(content: IntroScreenContent) -> Self {
        Self { cursor: 0, content }
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            _ => Command::none(),
        }
    }
}
