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

pub const COLS: usize = 96;
/// The longest command line the shell takes. Wider than any screen: a
/// command wraps on screen, it is not cut off.
pub const LINE_MAX: usize = 1024;
/// History lines each tab keeps.
pub const SCROLLBACK_ROWS: usize = 3000;
pub const VISIBLE_ROWS: usize = 40;
pub const HISTORY_DEPTH: usize = 32;
pub const MIN_FONT_SCALE: u32 = 1;
pub const MAX_FONT_SCALE: u32 = 6;
