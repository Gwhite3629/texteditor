use crate::App;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
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
        let str: String = self
            .cmdbuf
            .iter()
            .map(|k| keycode_to_text(k).pop().unwrap())
            .collect();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(
                Line::from(
                    " Mode: ".to_string()
                        + self
                            .status_text
                            .first()
                            .unwrap()
                            .clone()
                            .to_string()
                            .as_str()
                        + " "
                        + str.as_str()
                        + format!(" {},{}", self.text.pos().x, self.text.pos().y).as_str()
                        + format!(
                            " |({},{})-({},{})|",
                            self.text.highlight.p1.x,
                            self.text.highlight.p1.y,
                            self.text.highlight.p2.x,
                            self.text.highlight.p2.y
                        )
                        .as_str(),
                )
                .left_aligned(),
            )
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines = Vec::new();
        for (i, mut l) in self.text.iter().enumerate() {
            if self.text.highlight.p1 != self.text.highlight.p2 {
                if (i == self.text.highlight.p1.y as usize)
                    & (self.text.highlight.p1.y == self.text.highlight.p2.y)
                {
                    // Highlight span that is less than an entire line long, requires 2 splits
                    let (n1, t) = l.split_at_mut(self.text.highlight.p1.x as usize);
                    let (h, n2) = t.split_at_mut(
                        (self.text.highlight.p2.x - self.text.highlight.p1.x) as usize,
                    );
                    lines.push(Line::from(vec![
                        Span::styled(n1.to_string(), Style::default()),
                        Span::styled(h.to_string(), Style::default().bg(Color::Yellow)),
                        Span::styled(n2.to_string(), Style::default()),
                    ]));
                    continue;
                } else if i == self.text.highlight.p1.y as usize {
                    // Begin highlighting span
                    let (non, h) = l.split_at_mut(self.text.highlight.p1.x as usize);
                    lines.push(Line::from(vec![
                        Span::styled(non.to_string(), Style::default()),
                        Span::styled(h.to_string(), Style::default().bg(Color::Yellow)),
                    ]));
                    continue;
                } else if (i > self.text.highlight.p1.y as usize)
                    & (i < self.text.highlight.p2.y as usize)
                {
                    // Middle lines of highlight span
                    lines.push(Line::from(vec![Span::styled(
                        l,
                        Style::default().bg(Color::Yellow),
                    )]));
                    continue;
                } else if i == self.text.highlight.p2.y as usize {
                    // Finishing line of highlighting span
                    let (h, non) = l.split_at_mut(self.text.highlight.p2.x as usize);
                    lines.push(Line::from(vec![
                        Span::styled(h.to_string(), Style::default().bg(Color::Yellow)),
                        Span::styled(non.to_string(), Style::default()),
                    ]));
                    continue;
                }
            }
            lines.push(Line::from(l));
        }

        let counter_text = Text::from(lines);

        Paragraph::new(counter_text)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

fn keycode_to_text(code: &KeyCode) -> String {
    match code {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Enter => "\n".to_string(),
        KeyCode::Tab => "\t".to_string(),
        KeyCode::Backspace => "BACKSPACE".to_string(),
        KeyCode::Esc => "ESC".to_string(),
        KeyCode::Up => "UP".to_string(),
        KeyCode::Down => "DOWN".to_string(),
        KeyCode::Left => "LEFT".to_string(),
        KeyCode::Right => "RIGHT".to_string(),
        KeyCode::F(n) => format!("F{}", n),
        _ => String::new(),
    }
}
