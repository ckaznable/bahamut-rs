use bahamut::api::post::PostContent;
use ratatui::{
    widgets::{StatefulWidget, Block, Paragraph, Widget, Borders, Wrap},
    layout::{Layout, Constraint, Rect, Direction},
    buffer::Buffer,
    text::Spans
};

use super::state::PostPageState;

#[derive(Default)]
pub struct PostPageUI;

impl StatefulWidget for PostPageUI {
    type State = PostPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Min(0),
            ])
            .split(area);

        state.scroll_size(layout[1].height as usize);

        let top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Min(0),
            ])
            .split(layout[0]);

        // user
        let current = PostContent::default();
        let current = state.current().map_or(&current, |x|x);
        Paragraph::new(vec![
            Spans::from(current.user.id.as_ref()),
            Spans::from(current.user.name.as_ref()),
            Spans::from(current.user.carrer.to_string()),
            Spans::from(current.user.race.to_string()),
            Spans::from(format!("lv.{}", current.user.lv)),
        ])
        .block(Block::default().borders(Borders::ALL))
        .render(top[0], buf);

        Paragraph::new(vec![
            Spans::from(state.data.title.as_ref()),
            Spans::from(format!("{}樓", current.floor)),
            Spans::from(current.date.as_ref())
        ])
        .block(Block::default().borders(Borders::ALL))
        .render(top[1], buf);

        // desc
        let desc: Vec<Spans> = current.desc
            .iter()
            .skip(state.scroll_offset)
            .map(|s| Spans::from(s.to_owned()))
            .collect();
        Paragraph::new(desc)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL))
            .render(layout[1], buf);
    }
}
