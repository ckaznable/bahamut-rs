use ratatui::{widgets::{StatefulWidget, Cell, Row, Table, Block, Borders}, layout::{Layout, Constraint, Rect}, buffer::Buffer, style::{Style, Modifier}};

use super::state::SearchPageState;

#[derive(Default)]
pub struct SearchPageUI;

impl StatefulWidget for SearchPageUI {
    type State = SearchPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let header_cells = ["分類", "看版名稱"]
            .iter()
            .map(|h| Cell::from(*h));

        let header = Row::new(header_cells);

        let rows = state.items.iter().map(|item| {
            Row::new([
                Cell::from(item.platform.to_owned()),
                Cell::from(item.name.to_owned()),
            ])
        });

        let block = Block::default()
            .borders(Borders::ALL);

        Table::new(rows)
            .header(header)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .render(layout[0], buf, &mut state.state);
    }
}
