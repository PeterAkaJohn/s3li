use crossterm::event::KeyEventKind;

use super::component::{Component, ComponentProps, WithContainer};

#[derive(Debug)]
pub struct SimpleComponent {
    container_title: String,
}
impl SimpleComponent {
    pub fn new(container_title: String) -> SimpleComponent {
        Self { container_title }
    }
}
impl WithContainer<'_> for SimpleComponent {}
impl Component for SimpleComponent {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let explorer = self.with_container(&self.container_title, &props);
        f.render_widget(explorer, area);
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            _ => {}
        };
    }
}
