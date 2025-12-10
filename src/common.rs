use ratatui::widgets::Widget;

pub trait Screen: Widget + Tea {}
impl<T: Widget + Tea> Screen for T {}
pub trait Tea {
    fn view(&self);
    fn update(&mut self);
}
