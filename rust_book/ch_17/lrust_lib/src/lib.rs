// uses
pub use average_collection::AveragedCollection;
pub use blog::{
    blog_post::Post,
    blog_post_analog::{DraftPost, Post as PostAnalog, RequestReview},
};
pub use pseudo_gui::{Button, Draw, Screen};

// modules
pub mod average_collection;
pub mod blog;
pub mod pseudo_gui;
