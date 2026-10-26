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

//! A window resized from its frame, end to end: the drag on the border, the
//! app's new surface at the same origin, the window manager's rect. Before,
//! the window manager refused any size that overlapped another window (and
//! cascaded windows always do) after the app had drawn it, and moved the
//! origin of one that ran past the screen's edge; either way the window a
//! press reached was not the one on screen.

mod left_top;
mod overlap;
mod rig;
mod right_edge;
