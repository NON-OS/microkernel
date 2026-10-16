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

//! A holder of one claimed device, recording the order its device retires
//! in, and running its own teardown during the reset when asked.

use super::leaving::take_all;
use super::retire::Retire;
use super::types::Claim;

pub(super) const PID: u32 = 7;

#[derive(Default)]
pub(super) struct Holder {
    pub(super) log: Vec<&'static str>,
    pub(super) claims: Vec<Claim>,
    pub(super) granted: (u32, u32),
    pub(super) taken: (u32, u32),
    pub(super) mapped: bool,
    pub(super) freed: u32,
    pub(super) fault_during_reset: bool,
}

impl Retire for Holder {
    fn stop_bus_master(&mut self) {
        self.log.push("bm");
    }
    fn take_grants(&mut self) {
        self.taken = core::mem::take(&mut self.granted);
        self.log.push("take");
    }
    fn revoke_mmio(&mut self) {
        self.mapped = false;
        self.log.push("unmap");
    }
    fn reset(&mut self) {
        assert!(!self.mapped, "a BAR still mapped into the function in reset");
        self.log.push("flr");
        if self.fault_during_reset {
            // The faulting thread's teardown: the claims and grants it finds.
            assert_eq!(take_all(&mut self.claims, PID), Vec::<u64>::new(), "no second reset");
            assert_eq!(self.granted, (0, 0), "no grant left to free twice");
        }
    }
    fn free_frames(&mut self) {
        self.freed += core::mem::take(&mut self.taken).1;
        self.log.push("free");
    }
}

pub(super) fn holding() -> Holder {
    let claims = vec![Claim { pid: PID, device_id: 1, epoch: 3 }];
    Holder { claims, granted: (2, 4), mapped: true, ..Holder::default() }
}
