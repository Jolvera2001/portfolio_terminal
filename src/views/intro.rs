use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};

use crate::comms::{Command, Msg};

pub enum IntroMsg {
    CursorUp,
    CursorDown,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

        Paragraph::new(intro_text)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

impl IntroScreen {
    pub fn new(content: IntroScreenContent) -> Self {
        Self { cursor: 0, content }
    }

    pub fn update(&mut self, message: IntroMsg) -> Command<Msg> {
        match message {
            IntroMsg::CursorUp => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                Command::none()
            }
            IntroMsg::CursorDown => {
                if self.cursor < self.content.order.len() - 1 {
                    self.cursor += 1;
                }
                Command::none()
            }
        }
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            KeyCode::Char('w') | KeyCode::Up => {
                Command::perform(async { Msg::Intro(IntroMsg::CursorUp) })
            }
            KeyCode::Char('s') | KeyCode::Down => {
                Command::perform(async { Msg::Intro(IntroMsg::CursorDown) })
            }
            _ => Command::none(),
        }
    }
}
