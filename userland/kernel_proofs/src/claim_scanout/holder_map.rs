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

//! bootfb's map and store in the takeover model (L79), map_for_caller's steps:
//! the claim counted in while ready, the pages installed, the grant recorded
//! while the claim stands or the pages taken back, the count out; one store.

use super::events::{STORED_FB, STORED_SINK};
use super::give_back::RETIRE;
use super::holder::LOOKUP;
use super::stage_rule::{grantable, kept};
use super::world::{stage, Page};
use super::world_check::MARKED;
use super::world_state::World;

const INSTALL: u8 = 5;
const RECORD: u8 = 6;
const OUT: u8 = 7;
pub(super) const STORE: u8 = 8;

pub(super) fn step(w: &mut World, pc: u8) {
    let (epoch, r) = (w.round + 1, usize::from(w.round));
    let mine = |w: &World, f: fn(super::types::Stage) -> bool| {
        w.fb.is_some_and(|e| e.epoch == epoch && f(stage(e.stage)))
    };
    w.pc[2] = match pc {
        LOOKUP if mine(w, grantable) => {
            w.edit(epoch, |e| e.inflight += 1);
            INSTALL
        }
        LOOKUP => RETIRE,
        INSTALL => {
            w.page[r] = Page::Fb;
            RECORD
        }
        // grant::insert_live, asking claim::live under the grants' lock.
        RECORD => {
            if mine(w, kept) {
                w.grant[r] = true;
                w.mapped |= r == 0;
            } else {
                w.page[r] = Page::Nothing;
            }
            OUT
        }
        OUT => {
            w.edit(epoch, |e| e.inflight = e.inflight.saturating_sub(1));
            if w.page[r] == Page::Fb {
                STORE
            } else {
                RETIRE
            }
        }
        _ => {
            if w.page[r].shows_fb() && w.drawing() {
                w.fail(MARKED);
            }
            w.events |= if w.page[r].shows_fb() { STORED_FB } else { STORED_SINK };
            RETIRE
        }
    };
}
