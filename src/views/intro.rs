use ratatui::widgets::Widget;

pub struct IntroScreen {
    
}

impl Widget for &IntroScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}