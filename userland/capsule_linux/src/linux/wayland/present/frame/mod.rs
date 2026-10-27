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

//! The NØNOS frame around a guest's window: the title bar, the window
//! buttons, the border and the shadow every native window has, drawn by the
//! toolkit's own decorations (nonos_toolkit::decorations), so a Linux
//! program's window looks and moves like the desktop's own. A guest draws
//! only its content; the frame is the personality's, and its ground under
//! the content means a window is never see-through where its program drew
//! nothing. A window that fills the display has no frame.

mod buttons;
mod drag;
pub mod fill;
pub mod fresh;
pub mod geometry;
pub mod now;
pub mod paint;
pub mod pointer;
pub mod redraw;
pub mod state;

pub use geometry::Around;
pub use state::Frame;
