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

//! A legacy queue refused at bring-up: the receive queue may
//! already have its page number, in a grant the failed attempt releases
//! next, so the device is reset before the error goes back. A bring-up
//! whose queues are both taken is not reset.

use crate::constants::{RESET_MS, RING_SLOTS, RX_QUEUE_SIZE, TX_QUEUE_SIZE};
use crate::program_queues::program_queues;
use crate::regs::{fresh, with, Regs};

const PAGE: u64 = 0x20_0000;
const TX_PAGE: u64 = PAGE + 0x3000;
/// ACKNOWLEDGE, DRIVER and FEATURES_OK: where bring-up stands at the queues.
const LIVE: u8 = 0x0B;

#[test]
fn a_refused_transmit_queue_resets_the_device_after_the_receive_queue_had_its_ring() {
    let cases = [(1024, TX_PAGE), (0, TX_PAGE), (u16::MAX, TX_PAGE), (256, TX_PAGE + 8)];
    for (tx_max, tx_at) in cases.into_iter().chain([(256, 1 << 44)]) {
        fresh(RING_SLOTS);
        with(|m| (m.qmax[1], m.status) = (tx_max, LIVE));
        assert!(program_queues(Regs, PAGE, tx_at).is_err(), "tx {tx_max} at {tx_at:#x}");
        let after = with(|m| (m.pfn[0], m.status));
        assert_eq!(after, (Some(0x200), 0), "tx {tx_max} at {tx_at:#x}: not reset");
    }
}

#[test]
fn a_refused_receive_queue_resets_the_device_too() {
    fresh(1024);
    with(|m| m.status = LIVE);
    assert!(program_queues(Regs, PAGE, TX_PAGE).is_err());
    with(|m| assert_eq!((m.pfn, m.status), ([None, None], 0)));
}

#[test]
fn a_reset_that_never_completes_still_ends_the_attempt_by_the_clock() {
    fresh(RING_SLOTS);
    with(|m| (m.qmax[1], m.reset_lag) = (0, u32::MAX));
    assert!(program_queues(Regs, PAGE, TX_PAGE).is_err());
    // The refusal reads the status once to mark it FAILED; the reset then
    // reads it once per millisecond, and gives up when the time is spent.
    with(|m| assert_eq!(m.reset_lag, u32::MAX - 1 - RESET_MS as u32));
}

#[test]
fn queues_both_taken_leave_the_device_as_it_was() {
    fresh(RING_SLOTS);
    with(|m| m.status = LIVE);
    assert_eq!(program_queues(Regs, PAGE, TX_PAGE), Ok((RX_QUEUE_SIZE, TX_QUEUE_SIZE)));
    with(|m| assert_eq!((m.pfn, m.status), ([Some(0x200), Some(0x203)], LIVE)));
}
