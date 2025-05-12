#![allow(unused_imports)]

mod blocked;
pub use blocked::Blocked;

mod dyn_layout;
pub use dyn_layout::{DynLayout, ToDynLayout};

mod highlight_paragraph;
pub use highlight_paragraph::Hilighted;

mod text_box;
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::{Widget, WidgetRef}};
pub use text_box::TextBox;

pub mod utils;

