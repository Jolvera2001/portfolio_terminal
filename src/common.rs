use ratatui::widgets::Widget;

use crate::views::{contact::ContactScreen, guide::GuideScreen, intro::IntroScreen, projects::ProjectsScreen};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenID {
    Guide,
    Intro,
    Projects,
    Contact
}

pub enum ScreenType {
    Guide(GuideScreen),
    Intro(IntroScreen),
    Projects(ProjectsScreen),
    Contact(ContactScreen)
}

impl Widget for &ScreenType {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self {
            ScreenType::Guide(s) => s.render(area, buf),
            ScreenType::Intro(s) => s.render(area, buf),
            ScreenType::Projects(s) => s.render(area, buf),
            ScreenType::Contact(s) => s.render(area, buf),
        }
    }
}