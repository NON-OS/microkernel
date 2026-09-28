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

//! Extended colours after 38, 48 and 58, in either notation. A component
//! past 255 makes the colour invalid, not wrapped.

use crate::color::Color;

fn byte(v: u16) -> Option<u8> {
    u8::try_from(v).ok()
}

/// From a colon group: `[38, 5, n]`, `[38, 2, r, g, b]` or
/// `[38, 2, space, r, g, b]`.
pub(super) fn colour_from_group(g: &[u16]) -> Option<Color> {
    match g.get(1)? {
        5 => byte(*g.get(2)?).map(Color::Indexed),
        2 => {
            let rgb = if g.len() >= 6 { &g[3..6] } else { g.get(2..5)? };
            Some(Color::Rgb(byte(rgb[0])?, byte(rgb[1])?, byte(rgb[2])?))
        }
        _ => None,
    }
}

/// `5;n` or `2;r;g;b` spread over the groups after a 38 or 48. Returns the
/// colour and how many groups it took, so a malformed colour cannot swallow
/// what follows it beyond its own length.
pub(super) fn colour_from_spread(rest: &[&[u16]]) -> (Option<Color>, usize) {
    let v = |i: usize| rest.get(i).map(|g| g[0]);
    match v(0) {
        Some(5) => (v(1).and_then(byte).map(Color::Indexed), 2.min(rest.len())),
        Some(2) => {
            let c = (|| Some(Color::Rgb(byte(v(1)?)?, byte(v(2)?)?, byte(v(3)?)?)))();
            (c, 4.min(rest.len()))
        }
        Some(_) => (None, 1),
        None => (None, 0),
    }
}
