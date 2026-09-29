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

use alloc::vec::Vec;

use crate::browser::css::{Computed, Justify};

use super::super::track_widths::size_tracks;
use super::grid_occupy::Area;
use super::track_size::{per_track, tracks_for, Claim};

/* The (offset, height) of each of `nrows` rows holding items `heights`
 * tall (margins included) at `placed`, and the rows' total height. A
 * definite grid height `h` resolves percentage and fraction rows. */
pub(in super::super) fn row_sizes(
    s: &Computed,
    placed: &[Area],
    heights: impl Iterator<Item = i32>,
    nrows: usize,
    h: Option<i32>,
) -> (Vec<(i32, i32)>, i32) {
    let claims: Vec<Claim> = placed.iter().zip(heights).map(|(a, h)| (a.r, a.rs, h, h)).collect();
    let kind = |r: usize| if r < s.grid_row_n as usize { s.grid_rows[r] } else { s.grid_auto_rows };
    let t = tracks_for((0..nrows).map(kind), h, &per_track(&claims, nrows));
    size_tracks(t, &claims, h, s.row_gap as i32, Justify::Start)
}
