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

//! Bus mastering on the legacy bring-up (plan/50 C2). A device a dead
//! predecessor left live holds both rings' page numbers, in frames since
//! freed, and its reset does not clear them. Bus Master (`master`) goes on
//! only once the reset was read back, the handshake is done and both queues
//! are on this driver's grants, and never for a device refused before
//! that. It used to go on before the queues, and so before their refusals.

use std::cell::Cell;

use super::bus_master::noted;
use super::legacy_reset_tests::reset_then_handshaken;
use crate::constants::{RING_SLOTS, RX_QUEUE_SIZE, STATUS_ACKNOWLEDGE, STATUS_DRIVER};
use crate::constants::{STATUS_DRIVER_OK, STATUS_FEATURES_OK, TX_QUEUE_SIZE};
use crate::negotiate::negotiate;
use crate::program_queues::program_queues;
use crate::regs::{fresh, with, Regs};

const RX: u64 = 0x20_0000;
const TX: u64 = RX + 0x3000;
const SETTLED: u8 = STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK;

/// A device a predecessor left live, its queues offering `qmax` entries and
/// still on the predecessor's rings, which takes reads to reset, reset and
/// handshaken by `negotiate` as bring-up runs it.
fn left_live_and_handshaken(qmax: [u16; 2]) {
    fresh(RING_SLOTS);
    let (live, stale) = (SETTLED | STATUS_DRIVER_OK, Some(0xBAD));
    with(|m| (m.qmax, m.status, m.pfn, m.reset_lag) = (qmax, live, [stale; 2], 3));
    negotiate(Regs).expect("the handshake");
}

#[test]
fn a_device_left_live_masters_the_bus_only_reset_handshaken_and_on_this_drivers_rings() {
    left_live_and_handshaken([RING_SLOTS; 2]);
    let at_master = Cell::new(None);
    let seen = || with(|m| (m.status, reset_then_handshaken(&m.log), m.pfn));
    let master = || noted(&at_master, seen());
    assert_eq!(program_queues(Regs, RX, TX, master), Ok((RX_QUEUE_SIZE, TX_QUEUE_SIZE)));
    let rings = [Some((RX >> 12) as u32), Some((TX >> 12) as u32)];
    assert_eq!(at_master.get(), Some((SETTLED, true, rings)), "Bus Master on stale rings");
}

#[test]
fn a_device_refused_at_any_step_never_masters_the_bus() {
    // No receive queue, one too big for its grant (bhyve's), no transmit
    // queue, one past 256, one under it, a ring not page aligned, a page
    // number past 32 bits.
    let sizes = [[0, 256], [1024, 256], [256, 0], [256, 512], [256, 128]];
    let rings = [([256; 2], TX + 8), ([256; 2], 1 << 44)];
    for (qmax, tx) in sizes.map(|q| (q, TX)).into_iter().chain(rings) {
        left_live_and_handshaken(qmax);
        let mastered = Cell::new(None);
        assert!(program_queues(Regs, RX, tx, || noted(&mastered, ())).is_err(), "{qmax:?}");
        assert!(mastered.get().is_none(), "Bus Master on a refusal path: {qmax:?} tx {tx:#x}");
        with(|m| assert_eq!(m.status, 0, "{qmax:?} tx {tx:#x}: not left reset"));
    }
}
