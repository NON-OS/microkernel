// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Full screen for a guest window, as xdg_toplevel says it. Pure, so the
//! host proofs hold it.
//!
//! A guest window had one size for good: the first buffer's. Nothing told
//! the guest the window could be any other, nothing heard it ask, and the
//! window manager never learned it was meant to fill the display, so the
//! dock stayed over it. Now the guest is told the size and the state in a
//! configure, and the window takes them with the first buffer committed
//! after the guest acks that configure, never before: a buffer drawn for
//! the old size is shown where the window was.
//!
//! Maximised and full screen take the same rect, the green button's: the
//! whole width from the foot of the menubar down to the bottom edge. This
//! desktop has no smaller maximised, and the window manager is told both as
//! full screen, so the dock hides for either.

pub mod mode;
pub mod place;
mod state;
mod steps;

pub use mode::{Configure, Mode, Rect};
pub use place::origin;
pub use state::Fit;
