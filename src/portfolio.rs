use std::{collections::HashMap, env, fs, path::Path};

use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, symbols::border, text::Line, widgets::Block};

use crate::{
    common::{ScreenID, ScreenType},
    comms::{Command, Msg},
    views::{
        contact::ContactScreen, guide::GuideScreen, intro::{IntroScreen, IntroScreenContent}, projects::ProjectsScreen,
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
        let content = fetch_files().unwrap_or_else(|_| IntroScreenContent::default());

        screens.insert(ScreenID::Guide, ScreenType::Guide(GuideScreen::new()));
        screens.insert(ScreenID::Intro, ScreenType::Intro(IntroScreen::new(content)));
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
                    KeyCode::Char('2') => Command::perform(async {
                        Msg::Global(GlobalMsg::Navigate(ScreenID::Intro))
                    }),
                    KeyCode::Char('3') => Command::perform(async {
                        Msg::Global(GlobalMsg::Navigate(ScreenID::Projects))
                    }),
                    KeyCode::Char('4') => Command::perform(async {
                        Msg::Global(GlobalMsg::Navigate(ScreenID::Contact))
                    }),
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

fn fetch_files() -> Result<IntroScreenContent, eyre::Error> {
    let project_root = env::current_dir()?;
    let assets_path = project_root.join("assets");

    let intro_content = match fs::read_to_string(assets_path.join("intro_screen_content.ron")) {
        Ok(content) => {
            content
        },
        Err(e) => {
            return Err(e.into())
        },
    };

    let intro_content_deser = match ron::from_str(&intro_content) {
        Ok(content) => {
            content
        },
        Err(e) => {
            return Err(e.into())
        },
    };

    Ok(intro_content_deser)
}