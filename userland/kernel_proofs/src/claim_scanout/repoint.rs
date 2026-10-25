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

//! The repoint of a take-back in the takeover model (L79),
//! remap_in_asid's one hold of the paging lock over the drained grants.

use super::knob::Knob;
use super::world::Page;
use super::world_check::CLOBBER;
use super::world_state::World;

/// repoint_user_mmio over the drained grants, unless the holder's tables
/// are freed (space_of finds none): a page still showing its grant's frame
/// shows the sink in its tables; NoTranslateCheck moves every page.
pub(super) fn repoint(w: &mut World, g: usize, k: Knob) -> bool {
    let (mut moved, drained) = (false, if w.freed { 0 } else { w.g[g].drained });
    for r in (0..2).filter(|r| drained & 1 << r != 0) {
        let p = w.page[r];
        if p == Page::Fb || k == Knob::NoTranslateCheck {
            if matches!(p, Page::Nothing | Page::Other) {
                w.fail(CLOBBER);
            }
            (w.page[r], moved) = (Page::Stale, true);
        }
    }
    moved
}
