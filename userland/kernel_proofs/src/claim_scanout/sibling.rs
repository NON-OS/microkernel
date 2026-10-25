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

//! bootfb's sibling thread in the takeover model (L79), once B's first map
//! has returned: MkMmioUnmap of that grant (unmap_grant: the grant found,
//! its claim counted inside for Ready, Leaving or Exiting, the grant removed,
//! the pages unmapped, the count out; UnmapOutside counts nothing), or the
//! grant's address made to show something else outside the grant table.

use super::events::UNMAPPED;
use super::holder_map::STORE;
use super::knob::Knob;
use super::starts::Sib;
use super::types::Stage;
use super::world::{stage, Page, END};
use super::world_state::World;

const INSIDE: u8 = 1;
const REMOVE: u8 = 2;
const UNMAP: u8 = 3;
const OUT: u8 = 4;

pub(super) fn sibling(w: &mut World, k: Knob) -> bool {
    if w.round == 0 && w.pc[2] < STORE {
        return false;
    }
    w.pc[3] = match (w.cfg.sib, w.pc[3]) {
        (Sib::Stray, _) => {
            w.page[0] = if w.page[0] == Page::Fb { Page::Other } else { w.page[0] };
            END
        }
        // grant::find: an unknown grant (never made, or drained) is an error.
        (_, 0) if !w.mapped || !w.grant[0] => END,
        (_, 0) if k == Knob::UnmapOutside => REMOVE,
        (_, 0) => INSIDE,
        (_, INSIDE) => {
            let held = |s: Stage| matches!(s, Stage::Ready | Stage::Leaving | Stage::Exiting);
            if w.fb.is_some_and(|e| e.epoch == 1 && held(stage(e.stage))) {
                w.edit(1, |e| e.inflight += 1);
                w.inside = true;
            }
            REMOVE
        }
        (_, REMOVE) if w.grant[0] => {
            w.grant[0] = false;
            UNMAP
        }
        (_, REMOVE) => OUT,
        (_, UNMAP) => {
            (w.page[0], w.events) = (Page::Nothing, w.events | UNMAPPED);
            OUT
        }
        _ => {
            if w.inside {
                w.edit(1, |e| e.inflight = e.inflight.saturating_sub(1));
            }
            w.inside = false;
            END
        }
    };
    true
}
