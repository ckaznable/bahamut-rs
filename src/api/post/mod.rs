mod comment;
mod content;
mod page;

#[allow(clippy::module_inception)]
mod post;

pub type PostDescription = Vec<String>;

pub use comment::PostComment;
pub use content::{CommentReadable, PostContent};
pub use page::{PostPage, PostPageRef, PostPageUrlParameter};
pub use post::Post;
