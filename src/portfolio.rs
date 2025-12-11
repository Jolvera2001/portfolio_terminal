use std::collections::HashMap;

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{
    common::{ScreenID, ScreenType},
    comms::{Command, Msg},
    views::{
        contact::ContactScreen, guide::GuideScreen, intro::IntroScreen, projects::ProjectsScreen,
    },
};

pub enum GlobalMsg {
    Quit,
    Navigate(ScreenID),
    KeyPress(KeyEvent),
}

pub struct Portfolio {
    pub screens: HashMap<ScreenID, ScreenType>,
    pub current_screen: Option<ScreenID>,
    pub running: bool,
}

impl Portfolio {
    pub fn new() -> Self {
        let mut screens = HashMap::new();

        screens.insert(ScreenID::Guide, ScreenType::Guide(GuideScreen::new()));
        screens.insert(ScreenID::Intro, ScreenType::Intro(IntroScreen::new()));
        screens.insert(
            ScreenID::Projects,
            ScreenType::Projects(ProjectsScreen::new()),
        );
        screens.insert(ScreenID::Contact, ScreenType::Contact(ContactScreen::new()));

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

    pub fn update(&mut self, msg: Msg) -> Command<Msg> {
        match msg {
            Msg::Global(global_msg) => todo!(),
        }
    }
}
