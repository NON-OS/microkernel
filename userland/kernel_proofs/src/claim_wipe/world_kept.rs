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

//! A run's way back: held_run.rs `free_held_run`, which asks kept.rs `defer`
//! whether its wipe still keeps it; kept_settle.rs `settle_kept` at the
//! wipe's end (undo check 4 settles by device alone); and the registry's
//! let-go (grant_run.rs `free_unheld`).

use super::kept_rule::{on_let_go, on_settle, Act};
use super::world::World;
use super::world_cases::{LET_GO_FIRST, SETTLE_FIRST};
use super::world_knob::Knob;
use super::world_runs::RunM;

impl World {
    pub(super) fn free_held_run(&mut self, id: usize) {
        if !self.defer(id) {
            self.free_now(id);
        }
    }

    /// kept.rs `defer`: true while the run is kept and still waits.
    fn defer(&mut self, id: usize) -> bool {
        let Some(at) = self.kept.iter().position(|k| k.0 == id) else { return false };
        let (run, act) = on_let_go(self.kept[at].2);
        if act == Act::Free {
            self.kept.remove(at);
            return false;
        }
        self.seen[LET_GO_FIRST] += 1;
        self.kept[at].2 = run;
        true
    }

    /// kept_settle.rs `settle_kept`, keyed by (device, epoch).
    pub(super) fn settle(&mut self, epoch: u64, frees: bool) {
        let any = self.knob == Knob::SettleNoEpoch;
        let (mut go, seen) = (Vec::new(), &mut self.seen);
        self.kept.retain_mut(|k| {
            if k.1 != epoch && !any {
                return true;
            }
            let (run, act) = on_settle(k.2, frees);
            k.2 = run;
            seen[SETTLE_FIRST] += (act == Act::Wait) as u32;
            k.3 |= act == Act::Keep;
            if act == Act::Free {
                go.push(k.0);
            }
            act != Act::Free
        });
        go.into_iter().for_each(|id| self.free_now(id));
    }

    pub(super) fn let_go(&mut self) {
        let now = self.now;
        let due = |r: &RunM| r.registry && r.orphan_until <= now;
        let Some(id) = self.runs.iter().position(due) else { return };
        self.runs[id].registry = false;
        self.free_held_run(id);
    }
}
