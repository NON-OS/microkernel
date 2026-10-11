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

//! The stop before teardown.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::regs::{INT_ENABLE, INT_STATUS, RESET_ALL};
use super::engine::Host;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Stop everything the host does: a full reset ends any DMA, then bus
    /// power and clock go off. Called before the DMA regions are freed.
    pub fn quiesce(&mut self) {
        let _ = self.reset(RESET_ALL);
        self.power_off();
        self.io.w32(INT_ENABLE, 0);
        self.io.w32(INT_STATUS, u32::MAX);
    }
}
