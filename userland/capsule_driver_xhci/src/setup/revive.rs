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

//! A controller that failed once running (xHCI 1.2 sections 4.23.2 and
//! 4.24.1): a Host System Error or Host Controller Error halts it until it
//! is reset, and one gone from the bus reads all ones. Left alone, every
//! command went on timing out after 5 to 10 s, and port status showed every
//! port connected at speed 15.

use super::bring_up::bring_up;
use super::driver::Driver;
use super::teardown::{quiesce, Quiesced};
use crate::controller::{alloc_dcbaa, park::IrqWait, wait_cnr_clear};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::{controller_failed, usbsts_read};
use crate::rings::{command::CommandRing, event::EventRing};
use crate::slots::SlotTable;

/// Revivals one run of the capsule tries in all: a controller that keeps
/// failing is given up, not reset forever.
const MAX_REVIVALS: u8 = 3;

impl Driver {
    /// Whether the controller failed: USBSTS says so, or an abort found it
    /// wedged.
    pub fn failed(&self) -> bool {
        controller_failed(self.layout.op_base) || self.command_ring.wedged()
    }

    /// Bring the failed controller back from scratch: halted and reset, so
    /// it holds no pointer it was given, every slot dropped with what it
    /// held, then new rings and DCBAA, started and checked with a NOOP. One
    /// gone from the bus, or that would not halt, reset and get ready,
    /// stays dead.
    pub fn revive(&mut self) -> XhciResult<()> {
        if self.revivals >= MAX_REVIVALS {
            return Err(XhciError::ControllerDead);
        }
        self.revivals += 1;
        let op = self.layout.op_base;
        if usbsts_read(op) == u32::MAX || quiesce(op) != Quiesced::Reset {
            return Err(XhciError::ControllerDead);
        }
        // No operational register but USBSTS may be written until CNR clears
        // after the reset (xHCI 1.2 section 5.4.2), as at bring-up.
        wait_cnr_clear(op).map_err(|_| XhciError::ControllerDead)?;
        self.slots = SlotTable::new();
        let pool = self.dma_pool;
        self.dcbaa = alloc_dcbaa(&pool, self.layout.max_slots, self.scratchpads.array_phys())?;
        self.command_ring = CommandRing::new(&pool)?;
        let mut ring = EventRing::new(&pool)?;
        ring.irq = core::mem::replace(&mut self.event_ring.irq, IrqWait::new(0));
        self.event_ring = ring;
        if let Err(e) = bring_up(self) {
            let _ = quiesce(op);
            return Err(e);
        }
        self.dead = false;
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }
}
