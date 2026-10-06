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

use crate::browser::css::calc::split_top::words;
use crate::browser::css::computed::Computed;
use crate::browser::css::set_len::set_signed;
use crate::browser::css::sides::signed_sides;

/* Margin shorthand and sides, tracking auto on the horizontal axis so a
 * centered block can be recognised downstream. Margins are signed: a
 * negative margin pulls the box out past its container's content edge. */
pub(super) fn apply_margin(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "margin" => {
            let Some([t, r, b, l]) = signed_sides(value, fs) else {
                return true;
            };
            /* Note which horizontal sides read `auto`; signed_sides folded
             * them to 0. Words split at paren depth 0, like the sides. */
            let mut parts = [""; 4];
            let n = words(value).zip(parts.iter_mut()).map(|(w, p)| *p = w).count();
            let is_auto = |t: &str| t.eq_ignore_ascii_case("auto");
            let (r_auto, l_auto) = match n {
                1 => (is_auto(parts[0]), is_auto(parts[0])),
                2 | 3 => (is_auto(parts[1]), is_auto(parts[1])),
                _ => (is_auto(parts[1]), is_auto(parts[3])),
            };
            c.margin_right_auto = r_auto;
            c.margin_left_auto = l_auto;
            c.margin_auto_x = r_auto && l_auto;
            (c.margin_top, c.margin_right, c.margin_bottom, c.margin_left) = (t.0, r.0, b.0, l.0);
            c.margin_pml = [t.1, r.1, b.1, l.1];
        }
        "margin-top" => set_signed(&mut c.margin_top, &mut c.margin_pml[0], value, fs),
        "margin-right" => {
            c.margin_right_auto = value.trim().eq_ignore_ascii_case("auto");
            c.margin_auto_x = c.margin_left_auto && c.margin_right_auto;
            set_signed(&mut c.margin_right, &mut c.margin_pml[1], value, fs);
        }
        "margin-bottom" => set_signed(&mut c.margin_bottom, &mut c.margin_pml[2], value, fs),
        "margin-left" => {
            c.margin_left_auto = value.trim().eq_ignore_ascii_case("auto");
            c.margin_auto_x = c.margin_left_auto && c.margin_right_auto;
            set_signed(&mut c.margin_left, &mut c.margin_pml[3], value, fs);
        }
        _ => return false,
    }
    true
}
