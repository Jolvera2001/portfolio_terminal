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
            current_screen: Some(ScreenID::Guide),
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
            // screens manage keys and falls back to global
            Msg::Global(global_msg) => match global_msg {
                GlobalMsg::KeyPress(key) => match key.code {
                    KeyCode::Char('1') => Command::perform(async {
                        Msg::Global(GlobalMsg::Navigate(ScreenID::Guide))
                    }),
                    KeyCode::Char('2') => todo!(),
                    KeyCode::Char('3') => todo!(),
                    KeyCode::Char('4') => todo!(),
                    _ => {
                        if let Some(screen_id) = self.current_screen {
                            match self.screens.get_mut(&screen_id) {
                                Some(ScreenType::Guide(screen)) => screen.handle_key(key),
                                Some(ScreenType::Intro(screen)) => screen.handle_key(key),
                                Some(ScreenType::Projects(screen)) => screen.handle_key(key),
                                Some(ScreenType::Contact(screen)) => screen.handle_key(key),
                                None => Command::none(),
                            }
                        } else {
                            Command::none()
                        }
                    }
                },
                GlobalMsg::Navigate(screen_id) => {
                    if let Some(current) = self.current_screen {
                        if screen_id == current {
                            Command::none()
                        } else {
                            self.current_screen = Some(screen_id);
                            Command::none()
                        }
                    } else {
                        Command::none()
                    }
                }
            },

            // screen specific commands
            Msg::Guide(guide_msg) => {
                if self.current_screen == Some(ScreenID::Guide) {
                    if let Some(ScreenType::Guide(screen)) = self.screens.get_mut(&ScreenID::Guide)
                    {
                        screen.update(guide_msg)
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
        }
    }
}
