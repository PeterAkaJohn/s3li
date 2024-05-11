use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
};

pub struct Sources<'a> {
    pub component: ListComponent<'a, &'a str>,
}

impl<'a> Sources<'a> {
    pub fn new(items: Vec<&'a str>) -> Sources<'a> {
        Sources {
            component: ListComponent::new("Sources", items),
        }
    }
    pub fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        self.component.render(f, area, props)
    }
}
