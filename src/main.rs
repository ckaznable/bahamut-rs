extern crate lazy_static;

use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    io,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use bahamut::api::{
    board::BoardPage,
    post::{Post, PostComment, PostPage, PostPageUrlParameter},
    search::BoardSearch,
    CachedPage,
};
use channel::{DataRequestMsg, FetchDataMsg, PageData};
use crossterm::{
    event::{self, DisableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use tokio::runtime::Builder;
use ui::{
    key::handle_key,
    state::{AppState, ListStateInit, Page},
    ui,
};

mod channel;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx_req, rx_req) = channel::<DataRequestMsg>();
    let (tx_rev, rx_rev) = channel::<FetchDataMsg>();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // fetch thread
    let fetcher = run_fetcher(tx_rev, rx_req);

    // ui thread
    let app = AppState::new();
    let res = run_app(&mut terminal, app, tx_req.clone(), rx_rev);

    // close fetch thread
    tx_req.send(DataRequestMsg::End).unwrap_or(());
    fetcher.join().unwrap_or(());

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

        let poll_sec: f32 = if app.loading { 0.1 } else { 1000.0 };
        if let Ok(true) = event::poll(Duration::from_secs_f32(poll_sec)) {
            if let Event::Key(event) = event::read()? {
                if handle_key(&mut app, event, tx.clone()).is_quit() {
                    return Ok(());
                }
            };
        };

        if let Ok(v) = rx.try_recv() {
            match v {
                FetchDataMsg::SearchResult(v) => {
                    app.search.items(v);
                    app.search.init_select();
                    app.page = Page::Search;
                }
                FetchDataMsg::BoardPage(v) => {
                    app.board.items(v.items);
                    app.board.init_select();
                    app.board.last_page(v.max);
                    app.board.page(v.page);
                    app.page = Page::Board;
                }
                FetchDataMsg::PostPage(v) => {
                    if v.page == 1 {
                        app.post.data(v.items);
                        app.post.index(0);
                        app.post.page(v.page);
                        app.post.last_page(v.max);
                        app.page = Page::Post;
                    } else {
                        app.post.chain_posts(v.items.posts);
                        app.post.page(v.page);
                        app.post.next();
                    }
                }
                FetchDataMsg::CommentPage(v) => {
                    app.page = Page::Comment;
                    app.comment.items(v);
                }
            }

            app.loading = false;
        }
    }
}

fn run_fetcher(tx: Sender<FetchDataMsg>, rx: Receiver<DataRequestMsg>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut board_cache: HashMap<String, RefCell<BoardPage>> = HashMap::new();
        let mut post_cache: HashMap<String, RefCell<PostPage>> = HashMap::new();

        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async {
            loop {
                if let Ok(msg) = rx.recv() {
                    match msg {
                        DataRequestMsg::End => return,
                        DataRequestMsg::SearchResult(query) => {
                            let res = BoardSearch::get_search_result(query.as_ref());
                            if tx.send(FetchDataMsg::SearchResult(res)).is_err() {
                                println!("get search result error")
                            };
                        }

                        // board page request
                        DataRequestMsg::BoardPage(id, page, use_cache) => {
                            if use_cache {
                                if let Some(board_page) = board_cache.get(&id) {
                                    let mut board_page = board_page.borrow_mut();
                                    if let Some(board) = board_page.get_and_cache(page, false) {
                                        let items = board.post();
                                        let page_data = PageData {
                                            page,
                                            items,
                                            max: board_page.max,
                                        };
                                        tx.send(FetchDataMsg::BoardPage(page_data))
                                            .map_or((), |_| ());
                                        continue;
                                    }
                                }
                            }

                            let mut board = BoardPage::from_page(id.as_ref(), page);
                            board.init();

                            let items = match board.get_and_cache(page, false) {
                                Some(board) => board.post(),
                                None => vec![],
                            };

                            let page_data = PageData {
                                page,
                                items,
                                max: board.max,
                            };
                            board_cache.insert(id, RefCell::new(board));
                            tx.send(FetchDataMsg::BoardPage(page_data))
                                .map_or((), |_| ());
                        }

                        // post page request
                        DataRequestMsg::PostPage(url, page, use_cache) => {
                            let cache_key = url.to_owned();

                            if use_cache {
                                if let Some(post_page) = post_cache.get(&cache_key) {
                                    let mut post_page = post_page.borrow_mut();
                                    if let Some(post) = post_page.get_and_cache(page, false) {
                                        let page_data = PageData {
                                            page,
                                            items: post,
                                            max: post_page.max,
                                        };
                                        tx.send(FetchDataMsg::PostPage(page_data))
                                            .map_or((), |_| ());
                                        continue;
                                    }
                                }
                            }

                            let param = PostPageUrlParameter::try_from(url).unwrap();
                            let mut post_page = PostPage::try_from(param).unwrap();
                            post_page.init();

                            let items = match post_page.get_and_cache(page, false) {
                                None => Post::default(),
                                Some(post) => post,
                            };

                            let page_data = PageData {
                                page,
                                items,
                                max: post_page.max,
                            };
                            post_cache.insert(cache_key, RefCell::new(post_page));
                            tx.send(FetchDataMsg::PostPage(page_data))
                                .map_or((), |_| ())
                        }

                        // comment
                        DataRequestMsg::CommentPage(id, c_id) => {
                            let res = match PostComment::get_comment(id.to_owned(), c_id.to_owned())
                            {
                                Ok(v) => v,
                                Err(_) => vec![],
                            };

                            tx.send(FetchDataMsg::CommentPage(res)).map_or((), |_| ());
                        }
                    };
                };
            }
        })
    })
}
