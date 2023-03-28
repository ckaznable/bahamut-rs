use ratatui::{
    widgets::{StatefulWidget, Block, ListItem, List, Borders, Paragraph, Widget},
    layout::{Layout, Constraint, Rect},
    buffer::Buffer,
    text::Spans, style::{Style, Modifier, Color}
};

use super::state::{SearchPageState, InputMode};

#[derive(Default)]
pub struct SearchPageUI;

impl StatefulWidget for SearchPageUI {
    type State = SearchPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .horizontal_margin(4)
            .split(area);

        // input
        let block = Block::default()
            .borders(Borders::ALL)
            .title("看板搜尋");

        let width = layout[0].width.max(3) - 3;
        let scroll = state.input.visual_scroll(width as usize);
        Paragraph::new(state.input.value())
            .style(match state.mode {
                InputMode::Edit => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .scroll((0, scroll as u16))
            .block(block)
            .render(layout[0], buf);

        // search result
        let items: Vec<ListItem> = state.items
            .iter()
            .map(|item| {
                ListItem::new(vec![Spans::from(item.name.to_owned())])
            })
            .collect();

        let block = Block::default()
            .borders(Borders::ALL);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let list = List::new(items)
            .block(block)
            .highlight_style(selected_style);

        StatefulWidget::render(list, layout[1], buf, &mut state.state);
    }
}
