use bahamut::api::{search::SearchResult, board::BoardPost, post::{Post, PostContent, PostComment}};
use ratatui::widgets::ListState;
use tui_input::Input;


#[derive(Clone, PartialEq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Edit,
}

pub enum Page {
    Search,
    Board,
    Post,
    Comment,
}

pub trait CursorMoveable {
    fn index(&self) -> usize;
    fn max(&self) -> usize;
    fn next(&mut self);
    fn previous(&mut self);

    fn next_index(&self) -> usize {
        let i = self.index();
        if i >= self.max() - 1 {
            0
        } else {
            i + 1
        }
    }

    fn previous_index(&self) -> usize {
        let i = self.index();
        let max = self.max();

        if i > 0 {
            return i - 1
        }

        // i == 0
        if max > 0 {
            max - 1
        } else {
            i
        }
    }
}

pub trait ListStateInit<T> {
    fn lists(&self) -> &Vec<T>;
    fn state(&mut self) -> &mut ListState;

    fn init_select(&mut self) {
        if !self.lists().is_empty() {
            self.state().select(Some(0));
        } else {
            self.state().select(None);
        }
    }
}

pub struct AppState {
    pub page: Page,
    pub search: SearchPageState,
    pub board: BoardPageState,
    pub post: PostPageState,
    pub comment: PostCommentState,
    pub loading: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            page: Page::Search,
            search: SearchPageState::default(),
            board: BoardPageState::default(),
            post: PostPageState::default(),
            comment: PostCommentState::default(),
            loading: false,
        }
    }
}

#[derive(Default, Clone)]
pub struct SearchPageState {
    pub state: ListState,
    pub items: Vec<SearchResult>,
    pub mode: InputMode,
    pub input: Input,
    pub cursor: (u16, u16),
}

impl SearchPageState {
    pub fn items(&mut self, items: Vec<SearchResult>) {
        self.items = items;
    }

    pub fn mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn cursor(&mut self, cursor: (u16, u16)) {
        self.cursor = cursor;
    }
}

impl ListStateInit<SearchResult> for SearchPageState {
    fn lists(&self) -> &Vec<SearchResult> {
        &self.items
    }

    fn state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl CursorMoveable for SearchPageState {
    fn index(&self) -> usize {
        self.state.selected().map_or(1, |x|x)
    }

    fn max(&self) -> usize {
        self.items.len()
    }

    fn next(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(self.next_index()))
        }
    }

    fn previous(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(self.previous_index()))
        }
    }
}

#[derive(Default)]
pub struct BoardPageState {
    pub state: ListState,
    pub items: Vec<BoardPost>,
    pub id: String,
    pub name: String,
    pub last_page: u16,
    pub page: u16,
}

impl BoardPageState {
    pub fn id(&mut self, id: String) {
        self.id = id;
    }

    pub fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn items(&mut self, items: Vec<BoardPost>) {
        self.items = items;
    }

    pub fn last_page(&mut self, page: u16) {
        self.last_page = page;
    }

    pub fn page(&mut self, page: u16) {
        self.page = page;
    }
}

impl ListStateInit<BoardPost> for BoardPageState {
    fn lists(&self) -> &Vec<BoardPost> {
        &self.items
    }

    fn state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl CursorMoveable for BoardPageState {
    fn index(&self) -> usize {
        self.state.selected().map_or(1, |x|x)
    }

    fn max(&self) -> usize {
        self.items.len()
    }

    fn next(&mut self) {
        self.state.select(Some(self.next_index()))
    }

    fn previous(&mut self) {
        self.state.select(Some(self.previous_index()))
    }
}

#[derive(Default)]
pub struct PostPageState {
    pub data: Post,
    pub index: u16,
    pub page: u16,
    pub last_page: u16,
    pub url: String,
    pub scroll_offset: usize,
    pub scroll_size: usize,
}

impl PostPageState {
    pub fn data(&mut self, data: Post) {
        self.data = data;
    }

    pub fn chain_posts(&mut self, posts: Vec<PostContent>) {
        self.data.posts.extend(posts);
    }

    pub fn index(&mut self, index: u16) {
        self.index = index;
    }

    pub fn page(&mut self, page: u16) {
        self.page = page;
    }

    pub fn last_page(&mut self, page: u16) {
        self.last_page = page;
    }

    pub fn first(&mut self) {
        self.scroll_offset = 0;
        self.index = 0;
    }

    pub fn next(&mut self) -> Option<()> {
        let next_index = self.next_index();
        if self.index < self.data.posts.len() as u16 && self.data.posts.get(next_index).is_some() {
            self.scroll_offset = 0;
            self.index = next_index as u16;
            Some(())
        } else {
            None
        }
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.scroll_offset = 0;
            self.index -= 1;
        }
    }

    pub fn has_next(&self) -> bool {
        let next_index = self.next_index();
        if self.data.posts.get(next_index).is_some() {
            true
        } else {
            self.page < self.last_page
        }
    }

    fn next_index(&self) -> usize {
        ( self.index + 1 ) as usize
    }

    pub fn current(&self) -> Option<&PostContent> {
        self.data.posts.get(self.index as usize)
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scrollable() {
            self.scroll_offset += 1;
        }
    }

    pub fn scroll_size(&mut self, size: usize) {
        self.scroll_size = size;
    }

    fn scrollable(&self) -> bool {
        if let Some(desc) = self.current() {
            let desc = &desc.desc;

            !desc.is_empty() && desc.len() >= self.scroll_offset &&
            desc.len() - self.scroll_offset > self.scroll_size
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct PostCommentState {
    pub offset: usize,
    pub items: Vec<PostComment>,
    pub scroll_size: usize,
}

impl PostCommentState {
    pub fn items(&mut self, items: Vec<PostComment>) {
        self.items = items;
    }

    pub fn scroll_size(&mut self, size: usize) {
        self.scroll_size = size;
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() && self.scrollable() {
            self.offset += 1;
        }
    }

    pub fn previous(&mut self) {
        if !self.offset > 0 {
            self.offset -= 1;
        }
    }

    pub fn scrollable(&self) -> bool {
        self.items.len() - self.offset > self.scroll_size
    }

    pub fn init(&mut self) {
        self.offset = 0;
    }
}
