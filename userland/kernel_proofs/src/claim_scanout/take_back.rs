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

//! bootfb's claim taken back by a covering claim in the takeover model
//! (L79), scanout::take_back's steps: retired, the paths inside waited out,
//! its grants drained, each page still showing the framebuffer pointed at
//! the sink in one hold, the one TLB round, then the release; ReleaseFirst
//! puts the release before the repoint. A refused retire reads again.

use super::events::{REPOINTED, TOOK_BACK};
use super::knob::Knob;
use super::stage_rule::retired;
use super::take_order::{then, DRAIN, FLUSH, INFLIGHT, RELEASE, REPOINT, RETIRE, SETTLE};
use super::taker::PEEK;
use super::types::Stage;
use super::world::{code, stage, Page};
use super::world_check::SPIN;
use super::world_state::World;

pub(super) fn step(w: &mut World, g: usize, pc: u8, k: Knob) -> bool {
    let epoch = w.g[g].epoch;
    let mut to = then(k, pc);
    match pc {
        RETIRE => match w.fb.filter(|e| retired(stage(e.stage)).is_ok()) {
            Some(e) => {
                w.g[g].epoch = e.epoch;
                w.edit(e.epoch, |e| (e.stage, e.leavers) = (code(Stage::Leaving), e.leavers + 1));
            }
            // Refused: the stage must have changed since the read (SPIN if not).
            None => {
                if w.fb.is_some_and(|e| e.stage == w.g[g].seen) {
                    w.fail(SPIN);
                }
                to = PEEK;
            }
        },
        INFLIGHT if w.fb.is_some_and(|e| e.epoch == epoch && e.inflight > 0) => return false,
        INFLIGHT => {}
        DRAIN => w.g[g].drained = w.drain(epoch),
        REPOINT if !super::repoint::repoint(w, g, k) => to = then(k, FLUSH),
        REPOINT => w.events |= REPOINTED,
        // The round covers the run this take-back moved and no other page,
        // as the kernel flushes each grant's own range.
        FLUSH => {
            let drained = w.g[g].drained;
            for r in (0..2).filter(|r| drained & 1 << r != 0) {
                w.page[r] = if w.page[r] == Page::Stale { Page::Sink } else { w.page[r] };
            }
        }
        RELEASE if !w.named(epoch) => to = then(k, SETTLE),
        RELEASE => {}
        _ if w.settle(epoch) => w.events |= TOOK_BACK,
        _ => {}
    }
    w.pc[g] = to;
    true
}
