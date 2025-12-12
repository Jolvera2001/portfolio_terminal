use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, symbols::border, text::Line, widgets::Block};

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
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        let title = Line::from(" Welcome! ");
        let instructions = Line::from(vec![
            " Guide[1] |".into(),
            " Intro[2] |".into(),
            " Projects[3] |".into(),
            " Contact[4] ".into(),
        ]);

        let main = Block::bordered()
            .title(instructions.centered())
            .title_bottom(title.centered())
            .border_set(border::DOUBLE);

        let area = frame.area();
        let inner = main.inner(area);
        frame.render_widget(main, area);

        if let Some(screen_id) = self.current_screen {
            if let Some(screen) = self.screens.get(&screen_id) {
                frame.render_widget(screen, inner);
            }
        }
    }

    pub fn update(&mut self, msg: Msg) -> Command<Msg> {
        match msg {
            Msg::Global(global_msg) => match global_msg {
                GlobalMsg::KeyPress(key) => self.handle_key(key),
                GlobalMsg::Navigate(screen_id) => todo!(),
                _ => Command::none(),
            },
            Msg::Guide(guide_msg) => todo!(),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Command<Msg> {
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);

        match key.code {
            KeyCode::Char('Q') if shift => Command::perform(async { Msg::Global(GlobalMsg::Quit) }),
            KeyCode::Char('1') => todo!(),
            KeyCode::Char('2') => todo!(),
            KeyCode::Char('3') => todo!(),
            KeyCode::Char('4') => todo!(),
            _ => Command::none(),
        }
    }
}
