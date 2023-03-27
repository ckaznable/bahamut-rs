use ratatui::{
    widgets::{StatefulWidget, Block, ListItem, List},
    layout::{Layout, Constraint, Rect},
    buffer::Buffer,
    text::Spans
};

use super::state::SearchPageState;

#[derive(Default)]
pub struct SearchPageUI;

impl StatefulWidget for SearchPageUI {
    type State = SearchPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(4)
            .split(area);

        let items: Vec<ListItem> = state.items
            .iter()
            .map(|item| {
                ListItem::new(vec![Spans::from(item.name.to_owned())])
            })
            .collect();

        let block = Block::default();

        List::new(items)
            .block(block)
            .render(layout[0], buf, &mut state.state);
    }
}
