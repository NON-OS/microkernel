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

//! Config space as the model's function answers its reads. Every access is
//! judged against the window a reset write opened (I3), and every write a
//! wipe makes against the entry of a newer claim (I6).

use super::bits_rule::NO_SOFT_RESET;
use super::world::World;

impl World {
    pub(super) fn touch(&mut self, wipe_write: bool) {
        if self.now < self.dev.quiet_until {
            self.bad(3, "a config access inside a quiet window");
        }
        if wipe_write && self.entry.is_some() {
            self.bad(6, "a wipe wrote a function a newer claim holds");
        }
    }

    pub(super) fn pmcsr(&mut self) -> u16 {
        self.touch(false);
        let keeps = !self.dev.soft_d3;
        self.dev.pm_state | if keeps { NO_SOFT_RESET } else { 0 }
    }

    pub(super) fn tp(&mut self) -> bool {
        self.touch(false);
        self.now < self.dev.tp_until
    }

    pub(super) fn command(&mut self) -> u16 {
        self.touch(false);
        self.dev.cmd
    }

    pub(super) fn devctl(&mut self) -> u16 {
        self.touch(false);
        self.dev.devctl
    }

    /// Dword 0: its identity once back, else the retry status or all ones.
    pub(super) fn ident(&mut self) -> u32 {
        self.touch(false);
        let id = self.dev.caps.ident;
        match (self.now >= self.dev.ready_at, self.dev.retry) {
            (true, _) => id,
            (false, true) => (id & 0xFFFF_0000) | 1,
            (false, false) => u32::MAX,
        }
    }
}
