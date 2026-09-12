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

use crate::term::dimensions::COLS;

/// How much cut text is held for Ctrl-Y. A line cannot exceed COLS, so a
/// buffer that size can always hold whatever a single kill removed.
pub const KILL_CAP: usize = COLS;

pub struct Line {
    pub buf: [u8; COLS],
    pub len: usize,
    pub cursor: usize,
    /// The last text a kill key removed, waiting to be yanked back.
    pub killed: [u8; KILL_CAP],
    pub killed_len: usize,
}
