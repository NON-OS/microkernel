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

//! Bus mastering on the legacy bring-up, against the device
//! behind a port grant. One a dead predecessor left live holds its ring's
//! page number, in frames since freed, and its reset, which takes reads to
//! finish, does not clear it. Bus Master (`master`) goes on only once the
//! reset was read back, the handshake is done and the ring is this driver's
//! grant, and DRIVER_OK only after it; never for a device refused at its
//! ring. It used to go on before the ring and its refusals.

use std::cell::Cell;

use super::bus_master::noted;
use super::legacy_reset_tests::reset_then_handshaken;
use crate::constants::{LEGACY_MAX, STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK};
use crate::constants::{STATUS_FAILED, STATUS_FEATURES_OK};
use crate::init::{negotiate, start};
use crate::regs::{port, Regs};

const RING: u64 = 0x1234_5000;
const SETTLED: u8 = STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK;

/// A device offering `qnum` entries that a predecessor left live on its own
/// ring, reset and handshaken by `negotiate` as bring-up runs it.
fn left_live_and_handshaken(qnum: u16) -> Regs {
    let live = SETTLED | STATUS_DRIVER_OK;
    port(|p| (p.status, p.held, p.log, p.qnum, p.pfn) = (live, 3, Vec::new(), qnum, 0xBAD));
    negotiate(Regs::pio(1)).expect("the handshake");
    Regs::pio(1)
}

#[test]
fn a_device_left_live_masters_the_bus_only_reset_handshaken_and_on_this_drivers_ring() {
    let regs = left_live_and_handshaken(8);
    let at_master = Cell::new(None);
    let seen = || port(|p| (p.status, reset_then_handshaken(&p.log), p.pfn));
    assert_eq!(start(regs, RING, || noted(&at_master, seen())), Ok(8));
    let ours = (RING >> 12) as u32;
    assert_eq!(at_master.get(), Some((SETTLED, true, ours)), "Bus Master before the ring was ours");
    port(|p| assert_eq!(p.status, SETTLED | STATUS_DRIVER_OK, "DRIVER_OK after Bus Master"));
}

#[test]
fn a_device_refused_at_its_ring_never_masters_the_bus() {
    // No requestq, one past what the region holds, the most QueueNum can
    // say, a page past 32 bits.
    for (qnum, ring) in [(0, RING), (LEGACY_MAX + 1, RING), (u16::MAX, RING), (8, 1 << 44)] {
        let regs = left_live_and_handshaken(qnum);
        let mastered = Cell::new(None);
        assert!(start(regs, ring, || noted(&mastered, ())).is_err(), "qnum {qnum}");
        assert!(mastered.get().is_none(), "Bus Master on a refusal: qnum {qnum} ring {ring:#x}");
        let status = port(|p| p.status);
        assert_eq!(status & (STATUS_FAILED | STATUS_DRIVER_OK), STATUS_FAILED, "qnum {qnum}");
    }
}
