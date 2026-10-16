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

//! The driver the fuzz rounds run; what it does to its rings is
//! `driver_ring`, one round `run`.

use crate::constants::COMMAND_RING_TRBS;
use crate::controller::BulkPipes;
use crate::dma::DmaRegion;
use crate::event_ring::fixture::Fixture;
use crate::rings::command::CommandRing;
use crate::rings::transfer::TransferRing;
use crate::slots::{SlotResources, SlotTable};

/// Room kept free so a round's noise and its genuine event always fit.
pub(super) const ROUND_ROOM: usize = 6;
/// Usable TRBs on a command or transfer ring; the last holds the link.
pub(super) const RING_TRBS: u64 = COMMAND_RING_TRBS as u64 - 1;

/// One driver, kept across every round so the rings go round many times
/// and the parked events carry over.
pub(super) struct Driver {
    pub(super) fx: Fixture,
    pub(super) cmd: CommandRing,
    pub(super) cmd_issued: u64,
    pub(super) flush: CommandRing,
    pub(super) ep0: TransferRing,
    pub(super) desc: DmaRegion,
    pub(super) hid: SlotResources,
    pub(super) pipes: BulkPipes,
    pub(super) table: SlotTable,
}

impl Driver {
    pub(super) fn new() -> Self {
        let fx = Fixture::new();
        let cmd = fx.command_ring();
        let flush = fx.command_ring();
        let ep0 = fx.transfer_ring();
        let desc = fx.pool.alloc(64).expect("descriptor buffer");
        let hid = fx.hid_slot();
        let pipes = fx.bulk_pipes();
        Self { cmd, cmd_issued: 0, flush, ep0, desc, hid, pipes, table: SlotTable::new(), fx }
    }
}
