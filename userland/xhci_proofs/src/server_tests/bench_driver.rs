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

//! The driver the server runs, built by its own constructors around the
//! modelled window: 8 slots, 2 USB 2 root ports, 32-byte contexts, and
//! the event ring the producer serves.

use std::time::{Duration, Instant};

use nonos_devmodel::FakeBar;
use nonos_libc::MMIO_GRANT;

use super::hc::SLOTS;
use crate::conformance::model::{op_base, DOORBELL, PORT1};
use crate::constants::{PORTSC_CCS, PORTSC_PP};
use crate::controller::{alloc_dcbaa, program_command_ring, ControllerLayout, Scratchpads};
use crate::dma::DmaPool;
use crate::handles::BrokerHandles;
use crate::regs::cap::PortProtocols;
use crate::rings::{command::CommandRing, event::EventRing};
use crate::setup::Driver;
use crate::slots::SlotTable;

pub fn driver(bar: &FakeBar, pool: DmaPool, intr: u64, event_ring: EventRing) -> Driver {
    let layout = ControllerLayout {
        op_base: op_base(bar),
        doorbell_base: bar.base() + DOORBELL as u64,
        primary_intr_base: intr,
        max_slots: SLOTS,
        max_ports: 2,
        max_scratchpad: 0,
        max_psa: 0,
        context_size: 32,
        ports: PortProtocols::unknown(),
    };
    let mut command_ring = CommandRing::new(&pool).expect("command ring");
    program_command_ring(layout.op_base, &mut command_ring);
    Driver {
        handles: BrokerHandles::new(9, 1, MMIO_GRANT, bar.base(), 0),
        dcbaa: alloc_dcbaa(&pool, SLOTS, 0).expect("dcbaa"),
        scratchpads: Scratchpads::None,
        dma_pool: pool,
        command_ring,
        event_ring,
        layout,
        slots: SlotTable::new(),
        dead: false,
        generation: 0,
        revivals: 0,
    }
}

/// Power port 1 as bring-up powers every port, and wait for its device to
/// show the connect: a model thread scheduled late on a busy host must not
/// read as an empty port inside the driver's 20 ms power-good wait.
pub fn connected(bar: &FakeBar) {
    bar.present32(PORT1, PORTSC_PP);
    let deadline = Instant::now() + Duration::from_secs(5);
    while bar.wrote32(PORT1) & PORTSC_CCS == 0 && Instant::now() < deadline {
        std::thread::yield_now();
    }
}
