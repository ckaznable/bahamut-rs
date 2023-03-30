use ratatui::{widgets::{StatefulWidget, List, Block, ListItem, Borders, Widget}, layout::{Layout, Constraint, Alignment}, style::{Style, Modifier}, text::Spans};

use super::state::BoardPageState;

pub struct BoardPageUI;

impl StatefulWidget for BoardPageUI {
    type State = BoardPageState;

    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1)
            ])
            .horizontal_margin(1)
            .split(area);

        let block = Block::default()
            .title(format!("{} - 第{}頁", state.name, state.page))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let items: Vec<ListItem> = state.items
            .iter()
            .map(|item| {
                ListItem::new(vec![Spans::from(item.title.as_ref())])
            })
            .collect();

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let list = List::new(items)
            .block(block)
            .highlight_style(selected_style);
        StatefulWidget::render(list, layout[0], buf, &mut state.state);

        Block::default()
            .title(format!("<- {} / {} ->", state.page, state.last_page))
            .title_alignment(Alignment::Center)
            .render(layout[1], buf);
    }
}