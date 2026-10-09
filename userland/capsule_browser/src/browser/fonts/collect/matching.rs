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

use super::face::Face;

/// The face CSS font matching (CSS Fonts 4 5.2, step 4) picks for weight
/// `w`, among upright faces reaching Latin when there are such: one whose
/// range holds `w`; else for 400..=500 the nearest heavier up to 500, the
/// nearest lighter, then heavier past 500; below 400 lighter first; above
/// 500 heavier first. A tie goes to the face declared last.
pub(super) fn best<'a>(faces: &[&'a Face], w: u16) -> Option<&'a Face> {
    let upright = faces.iter().any(|f| !f.italic);
    let styled = |f: &&&Face| !upright || !f.italic;
    let latin = faces.iter().filter(styled).any(|f| f.latin);
    let pool = faces.iter().rev().filter(styled).filter(|f| !latin || f.latin);
    pool.min_by_key(|f| rank(f.weight, w)).copied()
}

fn rank((lo, hi): (u16, u16), w: u16) -> (u8, u16) {
    let (lighter, heavier) = (hi < w, lo > w);
    match w {
        _ if !lighter && !heavier => (0, 0),
        400..=500 if heavier && lo <= 500 => (1, lo - w),
        400..=500 if lighter => (2, w - hi),
        400..=500 => (3, lo - w),
        0..=399 if lighter => (1, w - hi),
        0..=399 => (2, lo - w),
        _ if heavier => (1, lo - w),
        _ => (2, w - hi),
    }
}
