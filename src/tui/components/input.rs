use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};

pub struct InputBlock {
    value: String,
    title: String,
    is_selected: bool,
}

impl InputBlock {
    pub fn new(value: String, title: String, is_selected: bool) -> Self {
        Self {
            value,
            title,
            is_selected,
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
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(input_container_style)
                    .border_type(BorderType::Rounded),
            );
        input_value.render(input_sections[0], buf);
        if self.is_selected {
            let starting_y = input_sections[0].y;
            let width = input_sections[0].width - 2; // need to remove 2 because of borders?
            let value_len = self.value.chars().count();
            // add 1 to increase offset when len equals width
            let offset = (value_len as u16 + 1).div_ceil(width);
            let y = starting_y + offset;
            let x = value_len as u16 % width;
            const WIDTH: u16 = 1;
            let last_position = Position {
                x: input_sections[0].x + 1 + x,
                y,
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
        if self.is_selected {
            let starting_y = input_sections[0].y;
            let width = input_sections[0].width - 2; // need to remove 2 because of borders?
            let value_len = self.value.chars().count();
            // add 1 to increase offset when len equals width
            let y = starting_y;
            let x = if (value_len as u16) < width - 1 {
                value_len as u16 % width
            } else {
                width - 1
            };
            const WIDTH: u16 = 1;
            let last_position = Position {
                x: input_sections[0].x + x,
                y,
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

fn fill<S: Into<Style>>(area: Rect, buf: &mut ratatui::prelude::Buffer, symbol: &str, style: S) {
    let style = style.into();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            buf.get_mut(x, y).set_symbol(symbol).set_style(style);
        }
    }
}
