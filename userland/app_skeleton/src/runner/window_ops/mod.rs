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
//! What changes the window itself: a move, a resize from the right or
//! bottom, a placing from the left or top, full screen, and a restore.

mod maximize;
mod move_window;
mod place_window;
mod resize_window;
mod restore;
mod tell_cursor;

pub(in crate::runner) use maximize::{follow_ask, toggle};
pub(in crate::runner) use move_window::apply_move;
pub(in crate::runner) use place_window::apply_place;
pub(in crate::runner) use resize_window::apply_resize;
pub(in crate::runner) use restore::restore;
pub(in crate::runner) use tell_cursor::tell_cursor;
