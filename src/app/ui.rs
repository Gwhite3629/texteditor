use crate::App;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Text Editor Simple ".bold());
        let instructions = Line::from(vec![
            " Edit ".into(),
            "<E>".blue().bold(),
            " Save ".into(),
            "<S>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(self.text.to_vec());

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
