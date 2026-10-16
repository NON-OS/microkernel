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

//! One pass of the route search, as deep as its limit: each connection it
//! follows is charged to the pin's budget, and a pass that runs the budget
//! out ends as one that found nothing (`search`).

use super::hop::usable;
use super::path::Path;
use super::widget::Codec;
use crate::constants::{WIDGET_TYPE_DAC, WIDGET_TYPE_MIXER, WIDGET_TYPE_SELECTOR};

/// Extend `p` from its last node towards a DAC, at most `limit` nodes long;
/// true once it ends at one.
pub(super) fn step(
    codec: &Codec,
    p: &mut Path,
    limit: usize,
    taken: &[Path],
    budget: &mut u32,
) -> bool {
    let at = p.len as usize - 1;
    let Some(w) = codec.get(p.nodes[at]) else { return false };
    if at + 1 >= limit {
        return false;
    }
    let fixed = taken.iter().find_map(|t| t.selection_of(codec, w.nid));
    for (i, &child) in w.connections().iter().enumerate() {
        let Some(left) = budget.checked_sub(1) else { return false };
        *budget = left;
        if child == 0 || p.nodes[..=at].contains(&child) || !usable(w, i, fixed) {
            continue;
        }
        let Some(c) = codec.get(child).filter(|c| !c.is_digital()) else { continue };
        (p.sel[at], p.nodes[at + 1], p.len) = (i as u8, child, (at + 2) as u8);
        let ty = c.ty();
        let through = ty == WIDGET_TYPE_MIXER || ty == WIDGET_TYPE_SELECTOR;
        if (ty == WIDGET_TYPE_DAC && c.plays_48k16())
            || (through && step(codec, p, limit, taken, budget))
        {
            return true;
        }
        p.len = (at + 1) as u8;
    }
    false
}
