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

//! The rings and slots a test builds from the fixture's pool, as the
//! driver and its handlers build them.

use super::fixture::{Fixture, BULK_IN_DCI, BULK_OUT_DCI, CONTEXT_SIZE, INT_DCI, SLOT};
use crate::contexts::{Place, Topology};
use crate::controller::BulkPipes;
use crate::rings::command::CommandRing;
use crate::rings::transfer::TransferRing;
use crate::slots::{InterruptEndpoint, SlotResources};

/// The payload of every interrupt endpoint the fixture sets up: one
/// boot-protocol report.
pub const REPORT: u16 = 8;

/// A device straight on root port `port` at protocol speed id `speed`.
pub fn root_place(port: u8, speed: u8) -> Place {
    Place { root_port: port, speed, topo: Topology::default() }
}

impl Fixture {
    pub fn command_ring(&self) -> CommandRing {
        CommandRing::new(&self.pool).expect("command ring")
    }

    pub fn transfer_ring(&self) -> TransferRing {
        TransferRing::new(&self.pool).expect("transfer ring")
    }

    /// An addressed slot with an interrupt-IN endpoint, set up as the
    /// alloc-transfer-ring handler does: its own ring and a buffer for its
    /// whole payload, `REPORT` bytes.
    pub fn hid_slot(&self) -> SlotResources {
        interrupt_slot(self, SLOT, &[INT_DCI])
    }

    pub fn bulk_pipes(&self) -> BulkPipes {
        BulkPipes::new(&self.pool, BULK_IN_DCI, BULK_OUT_DCI).expect("bulk pipes")
    }
}

/// An addressed `slot` with an interrupt-IN endpoint at each of `dcis`.
pub fn interrupt_slot(fx: &Fixture, slot: u8, dcis: &[u8]) -> SlotResources {
    let mut res = SlotResources::allocate(&fx.pool, CONTEXT_SIZE, slot, root_place(1, 3), false)
        .expect("slot resources");
    for &dci in dcis {
        res.interrupt.push(InterruptEndpoint {
            dci,
            ring: fx.transfer_ring(),
            buf: fx.pool.alloc(REPORT as u64).expect("report buffer"),
            len: REPORT,
            armed: None,
        });
    }
    res
}
