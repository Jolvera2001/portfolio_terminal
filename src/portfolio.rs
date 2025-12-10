use ratatui::{Frame};

use crate::common::{Screen, ScreenID};

pub struct Portfolio {
    pub screens: Vec<Box<dyn Screen>>,
    pub current_screen: Option<ScreenID>, 
    pub running: bool,
}

impl Portfolio {
    pub fn new() -> Self {
        Self {
            screens: vec![],
            current_screen: Some(ScreenID::Intro),
            running: true,
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        
    }

    pub fn update(&mut self) {

    }
}