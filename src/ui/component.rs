use anyhow::Result;
use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders, Padding},
    Frame,
};

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
    fn with_container(&self, container_title: &'a str, props: Option<ComponentProps>) -> Block<'a>
    where
        Self: Sized,
    {
        let box_style = match props {
            Some(ComponentProps { selected: true }) => Style::default().green(),
            _ => Style::default(),
        };
        Block::default()
            .borders(Borders::ALL)
            .border_style(box_style)
            .padding(Padding::horizontal(1))
            .title(Title::default().content(container_title))
    }
}
