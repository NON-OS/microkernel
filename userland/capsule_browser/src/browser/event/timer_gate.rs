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

use crate::browser::omnibox::Change;
use crate::browser::state::State;

use super::relayout::relayout;

/* While a load is in flight, timer relayouts keep at least this far apart,
 * so an animation loop cannot starve the steps that bring the page in. */
const LOADING_GAP_MS: u64 = 100;

/* Lay the page out after timers ran, but only when the DOM differs from
 * the one the current layout was made from. Timers that changed nothing (a
 * poll, an animation frame with no effect) cost a fingerprint, not a
 * restyle and layout. The layout is known to match when the fingerprint
 * and the display list are both the ones recorded at the last timer
 * relayout; any other relayout in between replaced the display list. */
pub(super) fn timer_relayout(state: &mut State, now: u64) -> bool {
    if !state.track.js_dirty {
        return false;
    }
    if state.loading() && now.wrapping_sub(state.track.js_relayout_ms) < LOADING_GAP_MS {
        return false;
    }
    state.track.js_dirty = false;
    state.track.js_relayout_ms = now;
    let Some(dom) = state.page_dom.as_ref() else {
        return false;
    };
    let print = dom_print(dom);
    if state.track.laid_print == Some((print, laid(state))) {
        return false;
    }
    relayout(state);
    state.track.laid_print = Some((print, laid(state)));
    state.mark(Change::Page);
    true
}

fn fold(h: u64, bytes: &[u8]) -> u64 {
    let h = bytes.iter().fold(h, |h, &b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3));
    (h ^ bytes.len() as u64).wrapping_mul(0x0100_0000_01b3)
}

/* Everything of the tree a layout reads: kind, links, tag, text, attrs. */
fn dom_print(dom: &crate::browser::dom::Dom) -> u64 {
    dom.nodes.iter().fold(0xcbf2_9ce4_8422_2325, |mut h, n| {
        h = fold(h, &[n.kind as u8]);
        h = fold(h, &(n.parent as u64).to_le_bytes());
        h = n.children.iter().fold(h, |h, c| fold(h, &(*c as u64).to_le_bytes()));
        h = fold(fold(h, n.tag.as_bytes()), n.text.as_bytes());
        n.attrs.iter().fold(h, |h, (k, v)| fold(fold(h, k.as_bytes()), v.as_bytes()))
    })
}

/* The display list on screen, by identity. */
fn laid(state: &State) -> u64 {
    let doc = state.box_doc.as_ref();
    doc.map_or(0, |d| fold(d.frags.as_ptr() as u64, &(d.frags.len() as u64).to_le_bytes()))
}
