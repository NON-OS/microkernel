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

//! A polled wait for one Normal Interrupt Status bit, bounded by the clock.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::regs::INT_STATUS;
use super::super::status::{judge, Seen};
use super::engine::Host;

pub(super) enum Waited {
    Timeout,
    Error(u16),
}

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    pub(super) fn wait(&self, want: u16, ms: u64) -> Result<(), Waited> {
        let d = Deadline::after(&self.clock, ms);
        loop {
            match judge(self.io.r32(INT_STATUS), want) {
                Seen::Done => {
                    self.io.w32(INT_STATUS, want as u32);
                    return Ok(());
                }
                Seen::Failed(e) => return Err(Waited::Error(e)),
                Seen::Pending => {}
            }
            if d.passed(&self.clock) {
                return Err(Waited::Timeout);
            }
            self.clock.relax();
        }
    }
}
