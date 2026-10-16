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

//! The holder's device retiring: each step recorded in order, checking
//! what must already be gone, and the holder's own teardown run during the
//! reset when asked.

use super::leaving::{claimable, done, take_all};
use super::retire::Retire;
use super::retire_holder::{Holder, PID};
use super::types::ClaimError;

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
    fn quiet_ports(&mut self) {
        self.ports_running = false;
        self.log.push("quiet");
    }
    fn reset(&mut self) {
        assert!(!self.mapped, "a BAR still mapped into the function in reset");
        assert!(!self.ports_running, "a port access still running into the function in reset");
        let refused = claimable(&self.claims, &self.busy, 1);
        assert_eq!(refused, Err(ClaimError::Busy), "a claim on the function in reset");
        self.log.push("flr");
        if self.fault_during_reset {
            // The faulting thread's teardown: the claims and grants it finds.
            assert_eq!(take_all(&mut self.claims, &mut self.busy, PID), Vec::<u64>::new());
            assert_eq!(self.granted, (0, 0), "no grant left to free twice");
        }
    }
    fn withdraw_dma(&mut self) {
        self.in_iommu = false;
        self.log.push("withdraw");
    }
    fn free_frames(&mut self) {
        assert!(!self.in_iommu, "a frame freed while the IOMMU still maps it");
        self.freed += core::mem::take(&mut self.taken).1;
        self.log.push("free");
    }
    fn detach(&mut self) {
        self.log.push("detach");
    }
    fn reopen(&mut self) {
        done(&mut self.busy, 1);
    }
}
