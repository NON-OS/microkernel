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
use crate::browser::css::computed::{Computed, Size};
use crate::browser::css::parse_grow::parse_grow;
use crate::browser::css::parse_size::parse_size;

/* flex shorthand: none, auto, or "<grow> [<shrink>] [<basis>]", split at
 * paren depth 0 so a calc() basis stays whole. The number-only forms mean
 * a zero basis ("flex: 1" shares the row equally), and an omitted shrink is
 * 1. none is 0 0 auto; auto is 1 1 auto; initial is 0 1 auto. */
pub(super) fn apply_flex_shorthand(c: &mut Computed, v: &str, fs: u32) {
    let (grow, shrink, basis) = match v {
        "none" => (0, 0, Size::Auto),
        "auto" => (100, 100, Size::Auto),
        "initial" => (0, 100, Size::Auto),
        _ => {
            let mut nums: [Option<u32>; 2] = [None, None];
            let mut basis = None;
            for tok in words(v) {
                let numeric = tok.parse::<f32>().is_ok();
                if numeric && nums[1].is_none() {
                    let slot = if nums[0].is_none() { 0 } else { 1 };
                    let Some(n) = parse_grow(tok) else { return };
                    nums[slot] = Some(n);
                } else if let (None, Some(s)) = (basis, parse_size(tok, fs)) {
                    basis = Some(s);
                } else if tok == "content" && basis.is_none() {
                    basis = Some(Size::Auto);
                } else {
                    return;
                }
            }
            let Some(grow) = nums[0].or(basis.map(|_| 100)) else { return };
            (grow, nums[1].unwrap_or(100), basis.unwrap_or(Size::Px(0)))
        }
    };
    (c.flex_grow, c.flex_shrink, c.flex_basis) = (grow, shrink, basis);
}
