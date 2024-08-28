use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};

trait WithCursor {
    fn get_inner_area(&self, section: &Rect) -> u16 {
        section.width - 2
    }
    fn get_x(&self, section: &Rect) -> u16;
    fn get_y(&self, section: &Rect) -> u16;
    fn is_visible(&self) -> bool;
    fn value_length(&self) -> u16;
    fn with_cursor(&self, section: Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.is_visible() {
            const WIDTH: u16 = 1;
            let last_position = Position {
                x: self.get_x(&section),
                y: self.get_y(&section),
            };
            let size = Size::new(WIDTH, 1);
            let area = Rect::from((last_position, size));
            fill(
                area,
                buf,
                "█",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::RAPID_BLINK),
            );
        }
    }
}

pub struct InputBlock {
    value: String,
    title: String,
    is_selected: bool,
    title_alignment: Alignment,
}

impl InputBlock {
    pub fn new(value: String, title: String, is_selected: bool) -> Self {
        Self {
            value,
            title,
            is_selected,
            title_alignment: Alignment::Center,
        }
    }

    pub fn with_title_alignment(self, title_alignment: Alignment) -> Self {
        Self {
            value: self.value,
            title: self.title,
            is_selected: self.is_selected,
            title_alignment,
        }
    }
}

impl Widget for InputBlock {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let input_container_style = if self.is_selected {
            Style::default().green()
        } else {
            Style::default()
        };
        let input_sections = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1)])
            .split(area);
        let input_value = Paragraph::new(self.value.to_string())
            .wrap(Wrap::default())
            .block(
                Block::new()
                    .title(self.title.as_str())
                    .title_alignment(self.title_alignment)
                    .borders(Borders::ALL)
                    .border_style(input_container_style)
                    .border_type(BorderType::Rounded),
            );
        input_value.render(input_sections[0], buf);
        self.with_cursor(input_sections[0], buf);
    }
}

impl WithCursor for InputBlock {
    fn get_x(&self, section: &Rect) -> u16 {
        let width = self.get_inner_area(section);
        let value_len = self.value_length();
        section.x + 1 + (value_len % width)
    }

    fn get_y(&self, section: &Rect) -> u16 {
        let starting_y = section.y;
        let width = self.get_inner_area(section);
        let value_len = self.value_length();
        let offset = (value_len + 1).div_ceil(width);
        starting_y + offset
    }

    fn is_visible(&self) -> bool {
        self.is_selected
    }

    fn value_length(&self) -> u16 {
        self.value.chars().count() as u16
    }
}

pub struct Input {
    value: String,
    is_selected: bool,
}
impl Input {
    pub fn new(value: String, is_selected: bool) -> Self {
        Self { value, is_selected }
    }
}

impl Widget for Input {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let input_sections = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1)])
            .split(area);
        let width: usize = (input_sections[0].width - 2).into(); // need to remove 2 because of borders?
        let value_len = self.value.chars().count();
        let horizonta_scroll = if value_len > width - 1 {
            value_len - width + 1
        } else {
            0
        };
        let input_value =
            Paragraph::new(self.value.to_string()).scroll((0, horizonta_scroll as u16));
        input_value.render(input_sections[0], buf);
        self.with_cursor(input_sections[0], buf);
    }
}

impl WithCursor for Input {
    fn get_x(&self, section: &Rect) -> u16 {
        let width = self.get_inner_area(section);
        let value_len = self.value_length();
        let x = if value_len < width - 1 {
            value_len % width
        } else {
            width - 1
        };
        x + section.x
    }

    fn get_y(&self, section: &Rect) -> u16 {
        section.y
    }

    fn is_visible(&self) -> bool {
        self.is_selected
    }

    fn value_length(&self) -> u16 {
        self.value.chars().count() as u16
    }
}

fn fill<S: Into<Style>>(area: Rect, buf: &mut ratatui::prelude::Buffer, symbol: &str, style: S) {
    let style = style.into();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            buf.get_mut(x, y).set_symbol(symbol).set_style(style);
        }
    }
}
