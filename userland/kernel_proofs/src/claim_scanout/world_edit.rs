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

//! What the takeover model's paths share (L79): bootfb's entry changed as
//! the kernel's stage rule says, a release settled, a drain of its grants,
//! and B's round ended.

use super::stage_rule::gone;
use super::types::Stage;
use super::world::{stage, Entry, END};
use super::world_state::World;

impl World {
    /// bootfb's entry at `epoch` changed by `f`. When the stage rule says it
    /// goes, it goes and the hold with it, as gone::remove does.
    pub(super) fn edit(&mut self, epoch: u8, f: impl FnOnce(&mut Entry)) -> bool {
        let Some(mut e) = self.fb.filter(|e| e.epoch == epoch) else {
            return false;
        };
        f(&mut e);
        let left = gone(stage(e.stage), u32::from(e.leavers), u32::from(e.inflight));
        self.fb = (!left).then_some(e);
        self.held &= !left;
        true
    }
    /// release::named: bootfb's claim at `epoch` is being released.
    pub(super) fn named(&self, epoch: u8) -> bool {
        let going = |s: Stage| matches!(s, Stage::Leaving | Stage::Exiting);
        self.fb.is_some_and(|e| e.epoch == epoch && going(stage(e.stage)))
    }
    /// release::settle: one release of the claim at `epoch` done.
    pub(super) fn settle(&mut self, epoch: u8) -> bool {
        self.named(epoch) && self.edit(epoch, |e| e.leavers = e.leavers.saturating_sub(1))
    }
    /// grant::drain_for_device: the rounds whose grant was made at `epoch`.
    pub(super) fn drain(&mut self, epoch: u8) -> u8 {
        let mut taken = 0;
        for r in 0..2 {
            if self.grant[r] && r as u8 + 1 == epoch {
                (self.grant[r], taken) = (false, taken | 1 << r);
            }
        }
        taken
    }
    /// B's thread is done with its round: the next one, or the end.
    pub(super) fn round_over(&mut self) {
        let again = self.cfg.again && self.round == 0;
        (self.round, self.pc[2]) = if again { (1, 0) } else { (self.round, END) };
    }
}
