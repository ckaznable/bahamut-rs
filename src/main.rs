use std::{error::Error, sync::mpsc::{channel, Sender, Receiver}, thread::{JoinHandle, self}, io, time::Duration, collections::HashMap};

use bahamut::api::{search::BoardSearch, board::BoardPage, CachedPage};
use channel::{FetchDataMsg, DataRequestMsg, PageData};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self}};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal};
use tokio::runtime::Builder;
use ui::{state::{AppState, ListStateInit, Page}, key::handle_key, ui};

mod ui;
mod channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx_req, rx_req) = channel::<DataRequestMsg>();
    let (tx_rev, rx_rev) = channel::<FetchDataMsg>();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // fetch thread
    let fetcher = run_fetcher(tx_rev, rx_req);

    // ui thread
    let app = AppState::new(tx_req.clone());
    let res = run_app(&mut terminal, app, tx_req.clone(), rx_rev);

    // close fetch thread
    tx_req.send(DataRequestMsg::End).unwrap_or_else(|_|());
    fetcher.join().unwrap_or_else(|_|());

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: AppState,
    tx: Sender<DataRequestMsg>,
    rx: Receiver<FetchDataMsg>,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Ok(true) = event::poll(Duration::from_secs_f32(0.5)) {
            if let Event::Key(event) = event::read()? {
                if handle_key(&mut app, event, tx.clone()).is_quit() {
                    return Ok(());
                }
            };
        };

        if let Ok(v) = rx.try_recv() {
            app.loading = false;

            match v {
                FetchDataMsg::SearchResult(v) => {
                    app.search.items(v);
                    app.search.init_select();
                    app.page = Page::Search;
                },
                FetchDataMsg::BoardPage(v) => {
                    app.board.items(v.items);
                    app.board.init_select();
                    app.board.last_page(v.max);
                    app.board.page(v.page);
                    app.page = Page::Board;
                }
            }
        };
    }
}

fn run_fetcher(tx: Sender<FetchDataMsg>, rx: Receiver<DataRequestMsg>) -> JoinHandle<()> {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    thread::spawn(move || {
        let mut board_cache: HashMap<String, BoardPage> = HashMap::new();

        rt.block_on(async {
            loop {
                if let Ok(msg) = rx.recv() {
                    match msg {
                        DataRequestMsg::End => return (),
                        DataRequestMsg::SearchResult(query) => {
                            let res = BoardSearch::get_search_result(query.as_ref());
                            if let Err(_) = tx.send(FetchDataMsg::SearchResult(res)) {
                                println!("get search result error")
                            };
                        }
                        DataRequestMsg::BoardPage(id, page) => {
                            if let Some(board_page) = board_cache.get(&id) {
                                if let Some(board) = board_page.get(page, false) {
                                    let items = board.post();
                                    let page_data = PageData { page, items, max: board_page.max };
                                    tx.send(FetchDataMsg::BoardPage(page_data)).map_or((), |_|());
                                    continue;
                                }
                            }

                            let mut board = BoardPage::from_page(id.as_ref(), page);
                            board.init();

                            let items = match board.get(page, false) {
                                Some(board) => board.post(),
                                None => vec!()
                            };

                            let page_data = PageData { page, items, max: board.max };
                            board_cache.insert(id, board);
                            tx.send(FetchDataMsg::BoardPage(page_data)).map_or((), |_|());
                        }
                    };
                };
            }
        })
    })
}
