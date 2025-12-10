use ratatui::Frame;

use crate::common::Screen;

pub struct Portfolio<'a> {
    pub Screens: Vec<Box<dyn Screen>>,
    pub mainFrame: &'a Frame,
}

impl<'a> Portfolio<'a> {
    pub fn new(frame: &'a Frame) -> Self {
        Self {
            Screens: vec![],
            mainFrame: frame,
        }
    }
}