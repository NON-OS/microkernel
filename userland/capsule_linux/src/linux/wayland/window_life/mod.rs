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

//! Which ends take a guest's window down, and in what order. Pure, so the
//! host proofs hold it.
//!
//! A guest's window was never taken down. Its toplevel destroyed, its
//! surface destroyed, its connection closed or its process ended, the
//! compositor went on drawing the last frame from memory this capsule had
//! let go, and the window manager kept the window: a ghost on the desktop
//! that a press still went to, with the dock hidden for good if it had
//! been full screen.

mod ends;
mod steps;

pub use ends::{ends_connection, takes_down, End, Shown};
pub use steps::{take_down, Steps};
