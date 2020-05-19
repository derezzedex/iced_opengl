//! Show toggle controls using checkboxes.
use crate::Renderer;

pub use iced_style::checkbox::{Style, StyleSheet};

/// A box that can be checked.
///
/// This is an alias of an `iced_native` checkbox with an `iced_wgpu::Renderer`.
pub type Checkbox<Message> = iced_native::Checkbox<Message, Renderer>;
