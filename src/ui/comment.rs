use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Paragraph, StatefulWidget, Widget, Wrap},
};

use super::state::PostCommentState;

pub struct CommentPageUI;

impl StatefulWidget for CommentPageUI {
    type State = PostCommentState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.scroll_size(area.height as usize);

        let name_style = Style::default().add_modifier(Modifier::REVERSED);
        let floor_style = Style::default().fg(Color::White);
        let items: Vec<Line> = state
            .items
            .iter()
            .skip(state.offset)
            .flat_map(|comment| {
                vec![
                    Line::from(vec![
                        Span::styled::<String>(format!("B{} ", comment.floor), floor_style),
                        Span::styled::<&str>(comment.nick.as_ref(), name_style),
                        Span::from(format!(": {}", comment.content)),
                    ]),
                    Line::from(""),
                ]
            })
            .collect();

        if items.is_empty() {
            Block::default().title("此篇沒有任何留言").render(area, buf);
        } else {
            Paragraph::new(items)
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }
}
