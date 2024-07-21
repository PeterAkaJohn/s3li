use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{self, Rect},
    style::{self, Stylize},
    widgets::{self, Block},
    Frame,
};

#[derive(Clone)]
pub struct ComponentProps {
    pub selected: bool,
}

pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    //   fn handle_events(&mut self, event: Option<Event>) -> Action {
    //     match event {
    //       Some(Event::Quit) => Action::Quit,
    //       Some(Event::Tick) => Action::Tick,
    //       Some(Event::Key(key_event)) => self.handle_key_events(key_event),
    //       Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
    //       Some(Event::Resize(x, y)) => Action::Resize(x, y),
    //       Some(_) => Action::Noop,
    //       None => Action::Noop,
    //     }
    //   }
    fn handle_key_events(&mut self, key: KeyEvent);
    //   fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
    //     Action::Noop
    //   }
    //   fn update(&mut self, action: Action) -> Action {
    //     Action::Noop
    //   }
    fn render(&mut self, f: &mut Frame, area: Rect, props: Option<ComponentProps>);
}

pub trait WithContainer<'a> {
    fn with_container(&self, container_title: &'a str, props: &Option<ComponentProps>) -> Block<'a>
    where
        Self: Sized,
    {
        let container = Block::default()
            .borders(widgets::Borders::ALL)
            .border_type(widgets::BorderType::Rounded)
            .padding(widgets::block::Padding::horizontal(1));
        match props {
            Some(ComponentProps { selected: true }) => container
                .border_style(style::Style::default().green())
                .title_alignment(layout::Alignment::Center)
                .title(widgets::block::Title::default().content(container_title)),
            _ => container.border_style(style::Style::default()),
        }
    }
}
