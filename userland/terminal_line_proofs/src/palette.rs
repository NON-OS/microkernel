// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's `palette` module, as far as the palette's painter and
//! pointer reach into it: the entries, the filter, the cap and the state.

pub use crate::entry::{Action, Entry, Kind};
pub use crate::filter::filter;
pub use crate::geom::MAX_ROWS;
pub use crate::state::Palette;
