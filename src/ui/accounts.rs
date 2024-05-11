use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
};

pub struct Accounts<'a> {
    pub component: ListComponent<'a, &'a str>,
}

impl<'a> Accounts<'a> {
    pub fn new(items: Vec<&'a str>) -> Accounts<'a> {
        Accounts {
            component: ListComponent::new("Accounts", items),
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
