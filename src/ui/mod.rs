pub mod state;
pub mod key;
pub mod search;
pub mod board;
pub mod loading;

use ratatui::{backend::Backend, Frame, layout::{Constraint, Direction, Layout, Rect}};

use self::{state::AppState, search::SearchPageUI, loading::Loading, board::BoardPageUI};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let size = f.size();

    // root layout
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(size);

    match app.page {
        state::Page::Search => {
            f.render_stateful_widget(SearchPageUI::default(), root[0], &mut app.search);
        },
        state::Page::Board => {
            f.render_stateful_widget(BoardPageUI, root[0], &mut app.board);
        },
        state::Page::Post => todo!(),
    };

    if app.loading {
        let area = centered_rect(10, 20, size);
        f.render_widget(Loading, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
        )
        .split(popup_layout[1])[1]
}