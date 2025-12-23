use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    text::{Line, Text},
    widgets::{Paragraph, Widget},
};
use serde::{Deserialize, Serialize};

use crate::comms::{Command, Msg};

pub enum ProjectMsg {
    CursorUp,
    CursorDown,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectItem {
    title: String,
    main_language: String,
    description: String,
}

pub struct ProjectsScreen {
    cursor: usize,
}

impl Widget for &ProjectsScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let intro_text = Text::from(vec![Line::from(vec!["Hello World!".into()])]);
        Paragraph::new(intro_text).render(area, buf);
    }
}

impl ProjectsScreen {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn update(&mut self, message: ProjectMsg) -> Command<Msg> {
        match message {
            ProjectMsg::CursorUp => {
                todo!();
                Command::none()
            }
            ProjectMsg::CursorDown => {
                todo!();
                Command::none()
            }
        }
    }

    pub fn handle_key(&self, key: KeyEvent) -> Command<Msg> {
        match key.code {
            KeyCode::Char('w') | KeyCode::Up => {
                Command::perform(async { Msg::Projects(ProjectMsg::CursorUp) })
            }
            KeyCode::Char('s') | KeyCode::Down => {
                Command::perform(async { Msg::Projects(ProjectMsg::CursorDown) })
            }
            _ => Command::none(),
        }
    }
}
