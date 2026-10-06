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

//! A driver with its event ring programmed, and the pieces each consumer of
//! the ring needs, all built by the driver's own constructors.

use nonos_devmodel::FakeBar;

use super::producer::Producer;
use crate::constants::TRB_BYTES;
use crate::controller::{program_event_ring, BulkPipes};
use crate::dma::DmaPool;
use crate::protocol::HID_REPORT_MAX;
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::rings::transfer::TransferRing;
use crate::slots::{InterruptEndpoint, SlotResources};

/// The slot and endpoints the fixture configures. Any other slot or
/// endpoint id in an event names something the driver never set up.
pub const SLOT: u8 = 2;
pub const EP0_DCI: u8 = 1;
pub const INT_DCI: u8 = 3;
pub const BULK_IN_DCI: u8 = 4;
pub const BULK_OUT_DCI: u8 = 5;
const CONTEXT_SIZE: u8 = 32;
/// One doorbell per slot, slot ids up to 255.
const DOORBELL_BYTES: usize = 256 * 4;
pub const TRB: u64 = TRB_BYTES as u64;

pub struct Fixture {
    pub pool: DmaPool,
    pub ring: EventRing,
    pub hc: Producer,
    pub doorbells: FakeBar,
}

impl Fixture {
    pub fn new() -> Self {
        let pool = DmaPool::new(1, 1);
        let ring = EventRing::new(&pool).expect("event ring");
        let mut hc = Producer::new();
        program_event_ring(hc.regs.base(), &ring);
        hc.start();
        hc.poison_past_segment();
        Self { pool, ring, hc, doorbells: FakeBar::new(DOORBELL_BYTES) }
    }

    pub fn intr(&self) -> u64 {
        self.hc.regs.base()
    }

    pub fn db(&self) -> u64 {
        self.doorbells.base()
    }

    pub fn command_ring(&self) -> CommandRing {
        CommandRing::new(&self.pool).expect("command ring")
    }

    pub fn transfer_ring(&self) -> TransferRing {
        TransferRing::new(&self.pool).expect("transfer ring")
    }

    /// An addressed slot with an interrupt-IN endpoint, set up as the
    /// alloc-transfer-ring handler does: its own ring and a HID_REPORT_MAX
    /// buffer.
    pub fn hid_slot(&self) -> SlotResources {
        let mut res =
            SlotResources::allocate(&self.pool, CONTEXT_SIZE, SLOT, 1, 3, false).expect("slot resources");
        res.interrupt.push(InterruptEndpoint {
            dci: INT_DCI,
            ring: self.transfer_ring(),
            buf: self.pool.alloc(HID_REPORT_MAX as u64).expect("report buffer"),
            armed: None,
        });
        res
    }

    pub fn bulk_pipes(&self) -> BulkPipes {
        BulkPipes::new(&self.pool, BULK_IN_DCI, BULK_OUT_DCI).expect("bulk pipes")
    }

    /// Where the driver's dequeue pointer is, checked to lie in the segment.
    pub fn dequeue_bus(&self) -> u64 {
        let at = self.ring.current_dequeue_phys();
        assert!(self.hc.segment_bus().contains(&at), "dequeue {at:#x} left the segment");
        assert_eq!(at % TRB, 0, "dequeue {at:#x} is not on a TRB boundary");
        at
    }
}
