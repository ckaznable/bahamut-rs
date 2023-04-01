use ratatui::{
    widgets::{StatefulWidget, Block, Paragraph, Widget, Borders},
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
        let current = state.current().unwrap();

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Min(0),
            ])
            .split(area);

        // user
        Paragraph::new(vec![
            Spans::from(current.user.id.as_ref()),
            Spans::from(current.user.name.as_ref()),
            Spans::from(current.user.carrer.to_string()),
            Spans::from(current.user.race.to_string()),
            Spans::from(format!("lv.{}", current.user.lv.to_string())),
        ])
        .block(Block::default().borders(Borders::ALL))
        .render(layout[0], buf);

        let right = Layout::default()
            .constraints([
                Constraint::Length(5),
                Constraint::Min(0),
            ])
            .split(layout[1]);

        Paragraph::new(vec![
            Spans::from(state.data.title.as_ref()),
            Spans::from(format!("{}樓", current.floor)),
            Spans::from(current.date.as_ref())
        ])
        .block(Block::default().borders(Borders::ALL))
        .render(right[0], buf);

        // desc
        let desc: Vec<Spans> = current.desc
            .iter()
            .map(|s| Spans::from(s.to_owned()))
            .collect();
        Paragraph::new(desc)
            .block(Block::default().borders(Borders::ALL))
            .render(right[1], buf);
    }
}
