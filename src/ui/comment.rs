use ratatui::{widgets::{StatefulWidget, Paragraph, Widget, Wrap, Block}, text::{Spans, Span}, style::{Style, Modifier, Color}, layout::Rect, buffer::Buffer};

use super::state::PostCommentState;

pub struct CommentPageUI;

impl StatefulWidget for CommentPageUI {
    type State = PostCommentState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.scroll_size(area.height as usize);

        let name_style = Style::default().add_modifier(Modifier::REVERSED);
        let floor_style = Style::default().fg(Color::White);
        let items: Vec<Spans> = state.items
            .iter()
            .skip(state.offset)
            .flat_map(|comment| vec![
                Spans::from(vec![
                    Span::styled::<String>(format!("B{} ", comment.floor), floor_style),
                    Span::styled::<&str>(comment.nick.as_ref(), name_style),
                    Span::from(format!(": {}", comment.content))
                ]),
                Spans::from(""),
            ])
            .collect();

        if items.is_empty() {
            Block::default()
                .title("此篇沒有任何留言")
                .render(area, buf);
        } else {
            Paragraph::new(items)
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }
}
