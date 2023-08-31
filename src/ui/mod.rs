pub mod board;
pub mod comment;
pub mod key;
pub mod loading;
pub mod post;
pub mod search;
pub mod state;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use self::{
    board::BoardPageUI,
    comment::CommentPageUI,
    loading::Loading,
    post::PostPageUI,
    search::SearchPageUI,
    state::{AppState, InputMode, Page},
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let size = f.size();

    match app.page {
        Page::Search => {
            f.render_stateful_widget(SearchPageUI, size, &mut app.search);
            if app.search.mode == InputMode::Edit {
                f.set_cursor(app.search.cursor.0, app.search.cursor.1);
            }
        }
        Page::Board => {
            f.render_stateful_widget(BoardPageUI, size, &mut app.board);
        }
        Page::Post => {
            f.render_stateful_widget(PostPageUI, size, &mut app.post);
        }
        Page::Comment => {
            f.render_stateful_widget(CommentPageUI, size, &mut app.comment);
        }
    };

    if app.loading {
        let y = if size.height < 18 {
            25
        } else if size.height > 25 {
            13
        } else {
            15
        };
        let area = centered_rect(10, y, size);
        f.render_widget(Loading, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
