pub fn add_white_space_till_width_if_needed(string: &str, width: usize) -> String {
    let mut line_item_label = format!("{: <25}", string);
    let line_item_label = if line_item_label.chars().count() < width {
        // calc difference and add whitespaces
        let diff = width - line_item_label.chars().count();
        let additions = " ".repeat(diff);
        line_item_label.push_str(&additions);
        line_item_label
    } else {
        line_item_label
    };
    line_item_label
}
