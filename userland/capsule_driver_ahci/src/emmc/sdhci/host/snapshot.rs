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

//! What the host registers read when asked, and the stop before teardown.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::regs::{
    CLOCK_CONTROL, HOST_CONTROL, INT_ENABLE, INT_STATUS, POWER_CONTROL, PRESENT_STATE, RESET_ALL,
};
use super::engine::Host;

/// What the host registers read when asked (CONTROLLER_INFO, PORT_LIST).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot {
    pub present: u32,
    pub host_control: u8,
    pub power: u8,
    pub clock: u16,
    pub int_status: u32,
}

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            present: self.io.r32(PRESENT_STATE),
            host_control: self.io.r8(HOST_CONTROL),
            power: self.io.r8(POWER_CONTROL),
            clock: self.io.r16(CLOCK_CONTROL),
            int_status: self.io.r32(INT_STATUS),
        }
    }

    /// Stop everything the host does: a full reset ends any DMA, then bus
    /// power and clock go off. Called before the DMA regions are freed.
    pub fn quiesce(&mut self) {
        let _ = self.reset(RESET_ALL);
        self.power_off();
        self.io.w32(INT_ENABLE, 0);
        self.io.w32(INT_STATUS, u32::MAX);
    }
}
