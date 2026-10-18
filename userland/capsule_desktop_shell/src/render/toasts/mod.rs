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

//! The toasts' panel: a column of recent notices above the dock, drawn last
//! in the chrome's frame so it sits over everything in it, and a window of
//! the shell's while it is up, so a press on it dismisses it rather than
//! reaching the window drawn under it.

mod layer;
mod metrics;
mod paint;
mod place;
mod press;
mod rows;

pub use layer::{paint_in_chrome, sync_toast_layer};
pub use press::toast_hit;
