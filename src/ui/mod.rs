pub mod state;
pub mod key;
pub mod search;
pub mod loading;

use ratatui::{backend::Backend, Frame, layout::{Constraint, Direction, Layout, Alignment, Rect}, widgets::{BorderType, Borders, Block}};

use self::{state::AppState, search::SearchPageUI, loading::Loading};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let size = f.size();

    // root layout
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(size);

    match app.page {
        state::Page::Search => {
            let search_page = SearchPageUI::default();
            f.render_stateful_widget(search_page, root[0], &mut app.search);
        },
        state::Page::Board => todo!(),
        state::Page::Post => todo!(),
    };

    if app.loading {
        let area = centered_rect(5, 20, size);
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
            .as_ref(),
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
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}