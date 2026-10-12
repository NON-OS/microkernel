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

//! A legacy device lays its rings out for its own QueueNumMax from the page
//! number it is given; the driver cannot shrink them. The grant and the
//! offsets fit a 256-entry ring only, so any other size is refused before
//! the page number is written, and the device is marked FAILED.

use crate::constants::{Q_RX, Q_TX, RING_SLOTS, STATUS_FAILED, VQ_REGION_SIZE};
use crate::program_queue::program_queue;
use crate::regs::{fresh, with, Regs};

const PAGE: u64 = 0x20_0000;

/// Where a legacy device's used ring ends for `q` entries (virtio 0.9.5,
/// 2.3: avail after 16q, used page aligned after that).
fn legacy_used_end(q: u64) -> u64 {
    let used = (16 * q + 6 + 2 * q).div_ceil(4096) * 4096;
    used + 6 + 8 * q
}

#[test]
fn only_a_256_entry_ring_fits_the_grant() {
    assert!(legacy_used_end(u64::from(RING_SLOTS)) <= VQ_REGION_SIZE as u64);
    for q in [512u64, 1024, 32768] {
        assert!(legacy_used_end(q) > VQ_REGION_SIZE as u64, "q {q} would fit");
    }
}

#[test]
fn a_queue_of_any_other_size_is_refused_before_the_pfn() {
    for qmax in [1024u16, 512, 257, 255, 128, 64, 1, 0, 32768, u16::MAX] {
        fresh(qmax);
        for q in [Q_RX, Q_TX] {
            assert!(program_queue(Regs, q, PAGE, 64).is_err(), "qmax {qmax} was accepted");
        }
        with(|m| {
            assert_eq!(m.pfn, [None, None], "qmax {qmax}: the device was given a ring");
            assert_ne!(m.status & STATUS_FAILED, 0, "qmax {qmax}: device not marked FAILED");
        });
    }
}

#[test]
fn a_256_entry_queue_is_programmed_and_primes_the_hint() {
    fresh(RING_SLOTS);
    assert_eq!(program_queue(Regs, Q_RX, PAGE, 64), Ok(64));
    assert_eq!(program_queue(Regs, Q_TX, PAGE + 0x3000, u16::MAX), Ok(RING_SLOTS));
    with(|m| assert_eq!(m.pfn, [Some(0x200), Some(0x203)]));
    with(|m| assert_eq!(m.status & STATUS_FAILED, 0));
}

#[test]
fn an_unaligned_or_unreachable_ring_is_refused() {
    for at in [PAGE + 8, PAGE + 0xFFF, 1 << 44, !0xFFF_u64] {
        fresh(RING_SLOTS);
        assert!(program_queue(Regs, Q_RX, at, 64).is_err(), "ring at {at:#x}");
        with(|m| assert_eq!(m.pfn[0], None));
        with(|m| assert_ne!(m.status & STATUS_FAILED, 0));
    }
}
