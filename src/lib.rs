//! BSP tiling layout for [`ratatui`].
//!
//! Handles splits, focus, resize, and movement. It only computes rectangles,
//! you render each pane and handle input.
//!
//! See [`ratatui-hypertile-extras`](https://docs.rs/ratatui-hypertile-extras)
//! for a ready-made runtime with plugins, keymaps, and a palette.
//!
//! ```
//! use ratatui::layout::Direction;
//! use ratatui_hypertile::{Hypertile, HypertileAction, EventOutcome};
//!
//! let mut layout = Hypertile::new();
//! let pane = layout.split_focused(Direction::Horizontal).unwrap();
//!
//! let outcome = layout.apply_action(HypertileAction::FocusNext);
//! assert_eq!(outcome, EventOutcome::Consumed);
//! ```

mod core;
mod engine;
mod input;
mod types;
mod widget;

pub use crate::core::{PaneId, StateError};
pub use crate::engine::{Hypertile, HypertileBuilder};
pub use crate::input::{
    EventOutcome, HypertileAction, HypertileEvent, KeyChord, KeyCode, Modifiers, MoveScope, Towards,
};
pub use crate::types::{PaneSnapshot, SplitPolicy};
pub use crate::widget::HypertileWidget;

pub mod raw {
    pub use crate::core::{HypertileState, Node, PaneId, StateError, collect_pane_ids};
}

pub mod prelude {
    pub use crate::{
        EventOutcome, Hypertile, HypertileAction, HypertileBuilder, HypertileEvent,
        HypertileWidget, KeyChord, KeyCode, Modifiers, MoveScope, PaneId, PaneSnapshot,
        SplitPolicy, StateError, Towards,
    };
}
