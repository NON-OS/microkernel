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

//! What a point names in the Browse listing, read off the same layout passes
//! the list and the grid draw from.

use super::grid_geom::cell_at;
use super::layout::{content_w, content_x};
use super::list_cols::{check_w, cols, head_h};
use super::pane::Pane;
use super::row_geom::row_at;
use super::target::Target;

/// In the grid, the card at `(x, y)`. In the list, the header strip first,
/// then the check box column of the row hit, and only then the row itself: a
/// row is hit by `y` alone, so the gutters either side of it reach it too.
pub fn browse_at(p: &Pane, x: u32, y: u32) -> Option<Target> {
    if p.grid {
        return cell_at(p, x, y).map(Target::Cell);
    }
    let c = cols(content_x(), content_w(p.win_w));
    if y >= p.row_top && y < p.row_top + head_h() {
        return c.cols.iter().position(|col| x >= col.x && x < col.x + col.w).map(Target::Sort);
    }
    let index = row_at(p, y)?;
    let boxed = x >= c.check_x && x < c.check_x + check_w();
    Some(if boxed { Target::Check(index) } else { Target::Row(index) })
}
