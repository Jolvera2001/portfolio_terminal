use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};

use crate::comms::{Command, Msg};

pub enum IntroMsg {
    CursorUp,
    CursorDown,
    ScrollUp,
    ScrollDown,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IntroScreenContent {
    sections: HashMap<String, Vec<String>>,
    order: Vec<String>,
}

pub struct IntroScreen {
    cursor: usize,
    content: IntroScreenContent,
    scroll: usize,
}

impl Widget for &IntroScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(40),
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .split(area);

        let text_area = chunks[3];

        let vertical_chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(90),
            Constraint::Fill(1),
        ])
        .split(text_area);

        let items: Vec<ListItem> = self
            .content
            .order
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

        let list = List::new(items).block(Block::bordered().title("About Me"));
        Widget::render(list, chunks[1], buf);

        let intro_text = self
            .content
            .order
            .get(self.cursor)
            .and_then(|k| self.content.sections.get(k))
            .map(|paragraphs| {
                let cleaned: Vec<String> = paragraphs
                    .iter()
                    .map(|p| {
                        p.lines()
                            .map(|line| line.trim())
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect();
                let text = cleaned.join("\n\n");
                Text::from(text)
            })
            .unwrap_or_else(|| Text::from(""));

        let selected_text = Paragraph::new(intro_text)
            .wrap(Wrap { trim: true })
            .block(
                Block::bordered()
                    .title("[Scroll up: J, Scroll down: K]")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::TOP | Borders::BOTTOM),
            )
            .scroll((self.scroll as u16, 0));

        Widget::render(selected_text, vertical_chunks[1], buf);
    }
}

impl IntroScreen {
    pub fn new(content: IntroScreenContent) -> Self {
        Self {
            cursor: 0,
            content,
            scroll: 0,
        }
    }

    pub fn update(&mut self, message: IntroMsg) -> Command<Msg> {
        match message {
            IntroMsg::CursorUp => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.scroll = 0;
                }
                Command::none()
            }
            IntroMsg::CursorDown => {
                if self.cursor < self.content.order.len() - 1 {
                    self.cursor += 1;
                    self.scroll = 0;
                }
                Command::none()
            }
            IntroMsg::ScrollUp => {
                self.scroll = self.scroll.saturating_sub(1);
                Command::none()
            }
            IntroMsg::ScrollDown => {
                self.scroll += 1;
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
            KeyCode::Char('j') | KeyCode::PageUp => {
                Command::perform(async { Msg::Intro(IntroMsg::ScrollUp) })
            }
            KeyCode::Char('k') | KeyCode::PageDown => {
                Command::perform(async { Msg::Intro(IntroMsg::ScrollDown) })
            }
            _ => Command::none(),
        }
    }
}
