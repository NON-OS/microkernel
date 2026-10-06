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

use crate::browser::css::Computed;

use super::super::ctx::Ctx;
use super::super::display_list::DisplayList;
use super::super::shift_down::shift_down;
use super::super::tree::BoxNode;
use super::flex_main::{basis_h, main_offsets};

/* A column item as laid: its fragments [a, b) and border-box height. */
pub(in super::super) struct Laid<'a> {
    pub node: &'a BoxNode,
    pub ab: [usize; 2],
    pub h: i32,
}

/* Settle a column's main axis after its items were stacked `used` px tall.
 * An item with a px flex-basis taller than its content is that tall. In a
 * box of definite height the free space then goes to the items that grow,
 * by their factors, or else to justify-content. Items move down by what
 * the ones above them gained; a grown item's own box gets taller. Returns
 * the column's content height. */
pub(in super::super) fn spread(
    laid: &[Laid],
    used: i32,
    s: &Computed,
    frags: &mut DisplayList,
    ctx: Ctx,
) -> i32 {
    let mut extra: Vec<i32> =
        laid.iter().map(|l| basis_h(&l.node.style).map_or(0, |b| (b - l.h).max(0))).collect();
    let used = extra.iter().fold(used, |u, e| u.saturating_add(*e));
    let free = ctx.cb.h.map_or(0, |h| h - used).max(0);
    let grow: i64 = laid.iter().map(|l| l.node.style.flex_grow as i64).sum();
    let (mut at, step, _) = match (free > 0, grow > 0) {
        (true, true) => {
            /* Factors summing under 1 hand out only that share. */
            for (e, l) in extra.iter_mut().zip(laid) {
                *e += (free as i64 * l.node.style.flex_grow as i64 / grow.max(100)) as i32;
            }
            (0, 0, 0)
        }
        (true, false) => main_offsets(free, laid.len() as i32, 0, 0, s.justify),
        _ => (0, 0, 0),
    };
    for (l, e) in laid.iter().zip(&extra) {
        shift_down(frags, l.ab[0], l.ab[1], at, ctx.clip);
        if let Some(f) = frags.get_mut(l.ab[0]).filter(|_| *e > 0 && l.ab[0] < l.ab[1]) {
            f.h = f.h.saturating_add(*e);
        }
        at = at.saturating_add(*e + step);
    }
    if free > 0 && (grow > 0 || at > 0) {
        used.saturating_add(free)
    } else {
        used
    }
}
