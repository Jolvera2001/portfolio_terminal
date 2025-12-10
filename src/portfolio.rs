use std::collections::HashMap;

use ratatui::Frame;

use crate::{
    common::{ScreenID, ScreenType},
    views::intro::IntroScreen,
};

pub struct Portfolio {
    pub screens: HashMap<ScreenID, ScreenType>,
    pub current_screen: Option<ScreenID>,
    pub running: bool,
}

impl Portfolio {
    pub fn new() -> Self {
        let screens = HashMap::new();

        screens.insert(ScreenID::Guide, ScreenType::Guide(IntroScreen::new()));
        screens.insert(ScreenID::Intro, ScreenType::Intro(IntroScreen::new()));
        screens.insert(ScreenID::Projects, ScreenType::Projects(IntroScreen::new()));
        screens.insert(ScreenID::Contact, ScreenType::Contact(IntroScreen::new()));


        Self {
            screens,
            current_screen: Some(ScreenID::Intro),
            running: true,
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        let area = frame.area();

        if let Some(screen_id) = self.current_screen {
            if let Some(screen) = self.screens.get(&screen_id) {
                frame.render_widget(screen, area);
            }
        }
    }

    pub fn update(&mut self) {}
}
