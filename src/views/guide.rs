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

    pub fn update() -> Command<Msg> {
        
    }
}


impl Widget for &GuideScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}
