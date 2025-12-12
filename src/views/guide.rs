use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::Widget};

use crate::comms::{Command, Msg};

pub enum GuideMsg {

}

pub struct GuideScreen {

}

impl GuideScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, msg: GuideMsg) -> Command<Msg> {
        match msg {
            _ => Command::none()
        }
    }

    pub fn handle_key(key: KeyEvent) -> Command<Msg> {
        todo!()
    }
}


impl Widget for &GuideScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}
