//! Navigate an endless amount of content with a scrollbar.
use crate::Renderer;

pub use iced_native::scrollable::State;
pub use iced_style::scrollable::{Scrollbar, Scroller, StyleSheet};

/// A widget that can vertically display an infinite amount of content
/// with a scrollbar.
///
/// This is an alias of an `iced_native` scrollable with a default
/// `Renderer`.
pub type Scrollable<'a, Message> =
    iced_native::Scrollable<'a, Message, Renderer>;
