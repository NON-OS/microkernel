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

//! The layout one splash keeps for its whole life, for its canvas size.

use crate::layout::{panel, status, warning, Area};

/// The panel, the warning's row and the status line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub panel: Area,
    pub warn: Area,
    pub status: Area,
}

/// The layout on a `w` x `h` surface.
pub fn layout(w: u32, h: u32) -> Layout {
    let p = panel(w, h);
    Layout { panel: p, warn: warning(p, w), status: status(w, h) }
}
