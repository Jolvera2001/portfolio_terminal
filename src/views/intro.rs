use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::comms::{Command, Msg};

pub struct IntroScreenContent {}
pub struct IntroScreen {}

impl Widget for &IntroScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let intro_text = Text::from(vec![Line::from(vec!["Hello World!".into()])]);
        Paragraph::new(intro_text).render(area, buf);
    }
}

impl IntroScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            _ => Command::none(),
        }
    }
}
