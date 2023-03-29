use ratatui::{widgets::{StatefulWidget, List, Block, ListItem, Borders}, layout::{Layout, Constraint, Alignment}, style::{Style, Modifier}, text::Spans};

use super::state::BoardPageState;

pub struct BoardPageUI;

impl StatefulWidget for BoardPageUI {
    type State = BoardPageState;

    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([
                Constraint::Min(0),
            ])
            .horizontal_margin(4)
            .split(area);

        let block = Block::default()
            .title(state.name.as_ref())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let items: Vec<ListItem> = state.items
            .iter()
            .map(|item| {
                ListItem::new(vec![Spans::from(item.title.as_ref())])
            })
            .collect();

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        List::new(items)
            .block(block)
            .highlight_style(selected_style)
            .render(layout[0], buf, &mut state.state);
    }
}