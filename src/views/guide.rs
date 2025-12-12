use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Widget},
};

use crate::comms::{Command, Msg};

pub enum GuideMsg {
    CursorUp,
    CursorDown,
}

pub struct GuideScreen {
    cursor: usize,
    options: Vec<String>,
}

impl Widget for &GuideScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let items: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let prefix = if i == self.cursor { "> " } else { " " };
                let style = if i == self.cursor {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}", prefix, option)).style(style)
            })
            .collect();

        let list = List::new(items);

        Widget::render(list, area, buf);
    }
}

impl GuideScreen {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            options: vec![
                "Hello world!".to_string(),
                "This is how the terminal navigates!".to_string(),
                "I hope you enjoy your stay :)".to_string(),
            ],
        }
    }

    pub fn update(&mut self, msg: GuideMsg) -> Command<Msg> {
        match msg {
            GuideMsg::CursorUp => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                Command::none()
            }
            GuideMsg::CursorDown => {
                if self.cursor < self.options.len() - 1 {
                    self.cursor += 1;
                }
                Command::none()
            }
        }
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            KeyCode::Char('w') => Command::perform(async { Msg::Guide(GuideMsg::CursorUp) }),
            KeyCode::Char('s') => Command::perform(async { Msg::Guide(GuideMsg::CursorDown)}),
            _ => Command::none(),
        }
    }
}

