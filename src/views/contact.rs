use ratatui::widgets::Widget;

pub struct ContactScreen {
    
}

impl Widget for &ContactScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        todo!()
    }
}


impl ContactScreen {
    pub fn new() -> Self {
        Self {}
    }
}