use ratatui::widgets::Widget;

pub struct ProjectsScreen {
    
}

impl Widget for &ProjectsScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}

impl ProjectsScreen {
    pub fn new() -> Self {
        Self {}
    }
}