use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct Loading;

impl Widget for Loading {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let block = Block::default()
            .style(Style::default().bg(Color::Gray))
            .borders(Borders::ALL);

        let text = "Loading...";

        Paragraph::new(text)
            .style(Style::default().fg(Color::Black))
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buf);
    }
}