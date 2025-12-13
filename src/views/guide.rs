use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph, Widget},
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
        // frame wide horizontal chunk
        let chunks = Layout::horizontal([
            Constraint::Percentage(50), 
            Constraint::Percentage(50)
            ])
            .split(area);

        // to center text
        let left_area = chunks[0];
        let vertical_chunks = Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ])
        .split(left_area);

        let text = Paragraph::new(
            "Welcome to the guide!\n
            Use W/S or Up/Down arrows to navigate the list on the right.
            Press 1-4 to switch screens,
            press Shift + Q to quit. (Note, you might need to press another key afterwards)",
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

        Widget::render(text, vertical_chunks[1], buf);

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

        let list = List::new(items).block(Block::bordered().title("Navigate me!"));

        Widget::render(list, chunks[1], buf);
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
            KeyCode::Char('w') | KeyCode::Up => Command::perform(async { Msg::Guide(GuideMsg::CursorUp) }),
            KeyCode::Char('s') | KeyCode::Down => Command::perform(async { Msg::Guide(GuideMsg::CursorDown) }),
            _ => Command::none(),
        }
    }
}
