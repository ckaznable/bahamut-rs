use ratatui::{
    widgets::{StatefulWidget, Block, ListItem, List, Borders, Paragraph, Widget},
    layout::{Layout, Constraint, Rect},
    buffer::Buffer,
    text::Spans, style::{Style, Modifier, Color}
};

use super::state::PostPageState;

#[derive(Default)]
pub struct PostPageUI;

impl StatefulWidget for PostPageUI {
    type State = PostPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
