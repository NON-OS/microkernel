// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's layout module, mirrored: its row arithmetic, and the window
//! split with the limits and types it reads, found here as `super::limits`.

pub use crate::layout_rows as rows;
pub use nonos_toolkit::widgets::Rect;

#[path = "../../../capsule_terminal/src/layout/compute.rs"]
pub mod compute;
#[path = "../../../capsule_terminal/src/layout/limits.rs"]
pub mod limits;
#[path = "../../../capsule_terminal/src/layout/types.rs"]
pub mod types;

pub use compute::compute;
pub use types::{Chrome, Layout, Rails};
