use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph, Widget},
};

use crate::comms::{Command, Msg};

pub enum GuideMsg {
    CursorUp,
    CursorDown,
}

pub struct GuideScreen {
    cols: usize,
    rows: usize,
    cursor: usize,
    options: Vec<String>,
}

impl Widget for &GuideScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let text = Paragraph::new(
            "Welcome to the guide!\n\n
            Use W/S or Up/Down arrows to navigate the list on the right.\n\n
            Press 1-4 to switch screens, \n\n
            press Shift + Q to quit. (Note, you might need to press another key afterwards)",
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

        Widget::render(text, chunks[0], buf);

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

        Widget::render(list, chunks[1], buf);
    }
}

impl GuideScreen {
    pub fn new() -> Self {
        Self {
            cols: 2,
            rows: 1,
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
            KeyCode::Char('s') => Command::perform(async { Msg::Guide(GuideMsg::CursorDown) }),
            _ => Command::none(),
        }
    }
}
