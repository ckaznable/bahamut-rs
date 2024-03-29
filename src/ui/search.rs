use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use super::state::{InputMode, SearchPageState};

#[derive(Default)]
pub struct SearchPageUI;

impl StatefulWidget for SearchPageUI {
    type State = SearchPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .horizontal_margin(4)
            .split(area);

        // input
        let block = Block::default().borders(Borders::ALL).title("看板搜尋");
        let width = layout[0].width.max(3) - 3;
        let scroll = state.input.visual_scroll(width as usize);
        let value = state.input.value();
        let input_value = match state.mode {
            InputMode::Edit => value,
            InputMode::Normal => {
                if value.is_empty() {
                    "輸入 'a'/'e'/'i'/'o' 開始搜尋"
                } else {
                    value
                }
            }
        };
        Paragraph::new(input_value)
            .style(match state.mode {
                InputMode::Edit => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .scroll((0, scroll as u16))
            .block(block)
            .render(layout[0], buf);

        state.cursor((
            // Put cursor past the end of the input text
            layout[0].x + ((state.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            layout[0].y + 1,
        ));

        // search result
        let items: Vec<ListItem> = state
            .items
            .iter()
            .map(|item| ListItem::new(vec![Line::from(item.name.as_ref())]))
            .collect();

        let block = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let list = List::new(items)
            .block(block)
            .highlight_style(selected_style);

        StatefulWidget::render(list, layout[1], buf, &mut state.state);
    }
}
