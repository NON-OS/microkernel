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

//! The DMA runs: mapped by a holder, then torn down by its unmap, a release
//! or an exit (teardown.rs), each kept for its function's wipe where the
//! function could still reach it.

use super::kept_rule::{keeps, Run};
use super::phase_rule::MS;
use super::world::World;
use super::world_knob::Knob;

#[derive(Clone, Copy, Debug)]
pub(super) struct RunM {
    pub pid: usize,
    pub epoch: u64,
    pub len: u64,
    pub confined: bool,
    /// Another process maps a surface over the run until then.
    pub orphan_until: u64,
    pub live: bool,
    /// The registry held it at its teardown and has not let go.
    pub registry: bool,
    pub frees: u8,
}

const MIB: u64 = 1 << 20;
const LENS: [u64; 7] = [4096, 1 << 16, 2 * MIB, 16 * MIB, 64 * MIB, 128 * MIB, 256 * MIB];

impl World {
    /// A holder maps a run: confined unless the function is untranslated or
    /// the draw says not; an unconfined one its queue may write.
    pub(super) fn map_run(&mut self, pid: usize, epoch: u64) {
        let len = LENS[self.draw(7) as usize];
        let confined = !self.dev.caps.untranslated && self.chance(2);
        let orphan_until = if self.chance(3) { self.now + MS * self.draw(2001) } else { 0 };
        let (live, registry, frees) = (true, false, 0);
        self.runs.push(RunM { pid, epoch, len, confined, orphan_until, live, registry, frees });
        if !confined {
            self.dev.queue.push(self.runs.len() - 1);
        }
    }

    /// teardown.rs `teardown`, the run given up by its domain: kept for the
    /// wipe if a release drains it unconfined (undo check 1 frees it here).
    pub(super) fn teardown(&mut self, id: usize, released: bool) {
        let now = self.now;
        let r = &mut self.runs[id];
        (r.live, r.registry) = (false, now < r.orphan_until);
        let (epoch, confined, held) = (r.epoch, r.confined, r.registry);
        if keeps(true, confined, released) && self.knob != Knob::FreeAtTeardown {
            self.kept.push((id, epoch, Run { let_go: !held, settled: false }, false));
        } else if !held {
            self.free_held_run(id);
        }
    }

    /// kept.rs `kept_bytes`, which enter.rs bounds.
    pub(super) fn kept_bytes(&self) -> u64 {
        self.kept.iter().filter(|k| k.3).map(|k| self.runs[k.0].len).sum()
    }
}
