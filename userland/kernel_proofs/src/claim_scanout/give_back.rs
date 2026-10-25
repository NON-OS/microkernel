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

//! bootfb's give_back in the takeover model (L79): retired, the paths
//! inside waited out, its grants drained and its own pages unmapped in its
//! own tables, then released; the entry goes with the last release and the
//! last path inside it. A refused retire gives nothing back.

use super::events::GAVE_BACK;
use super::stage_rule::retired;
use super::types::Stage;
use super::world::{code, stage, Page};
use super::world_state::World;

pub(super) const RETIRE: u8 = 9;
const INFLIGHT: u8 = 10;
const DRAIN: u8 = 11;
const UNMAP: u8 = 12;
const RELEASE: u8 = 13;

pub(super) fn step(w: &mut World, pc: u8) -> bool {
    let epoch = w.gb.0;
    match pc {
        RETIRE => match w.fb.filter(|e| retired(stage(e.stage)).is_ok()) {
            Some(e) => {
                w.gb = (e.epoch, 0);
                w.edit(e.epoch, |e| (e.stage, e.leavers) = (code(Stage::Leaving), e.leavers + 1));
                w.pc[2] = INFLIGHT;
            }
            None => w.round_over(),
        },
        INFLIGHT if w.fb.is_some_and(|e| e.epoch == epoch && e.inflight > 0) => return false,
        INFLIGHT => w.pc[2] = DRAIN,
        DRAIN => {
            w.gb.1 = w.drain(epoch);
            w.pc[2] = UNMAP;
        }
        UNMAP => {
            for r in (0..2).filter(|r| w.gb.1 & 1 << r != 0) {
                w.page[r] = Page::Nothing;
            }
            w.pc[2] = RELEASE;
        }
        RELEASE if w.named(epoch) => w.pc[2] = RELEASE + 1,
        RELEASE => w.round_over(),
        _ => {
            w.settle(epoch);
            w.events |= GAVE_BACK;
            w.round_over();
        }
    }
    true
}
