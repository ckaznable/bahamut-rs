pub mod state;
pub mod key;
pub mod search;

use ratatui::{backend::Backend, Frame, layout::{Constraint, Direction, Layout, Alignment}, widgets::{BorderType, Borders, Block}};

use self::{state::AppState, search::SearchPageUI};

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
}