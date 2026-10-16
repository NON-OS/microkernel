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

//! A ring never laps the controller. Both rings went back to their first
//! TRB on reaching the Link, so their Full checks could never fire, and
//! after a recovery that failed, or commands given up on a ring that was
//! not running, the driver wrote on over TRBs the controller had yet to
//! run. Now a ring holds at most 62 TRBs no completion or dequeue move has
//! retired, and refuses the next with the ring untouched.

use nonos_libc::dma_host;

use crate::dma::DmaPool;
use crate::error::XhciError;
use crate::rings::command::CommandRing;
use crate::rings::transfer::TransferRing;
use crate::trb::{read_volatile_at, Trb};

const ROOM: u32 = 62;

fn marked(n: u32) -> Trb {
    Trb { d0: 0x5EED_0000 | n, d1: 0, d2: 0, d3: 23 << 10 }
}

fn marker_at(phys: u64) -> u32 {
    let host = dma_host(phys).expect("a ring TRB");
    // SAFETY: `dma_host` named live host memory behind the ring.
    unsafe { read_volatile_at(host) }.d0
}

#[test]
fn a_transfer_ring_refuses_a_trb_past_those_the_controller_has_not_run() {
    let mut ring = TransferRing::new(&DmaPool::new(9, 1)).expect("ring");
    let first = ring.enqueue(marked(0)).expect("first");
    for n in 1..ROOM {
        ring.enqueue(marked(n)).expect("room");
    }
    assert_eq!(ring.enqueue(marked(ROOM)), Err(XhciError::TransferRingFull));
    assert_eq!(ring.enqueue(marked(ROOM + 1)), Err(XhciError::TransferRingFull));
    assert_eq!(marker_at(first), marked(0).d0, "the first TRB is still there");
    ring.retire();
    for n in ROOM..2 * ROOM {
        ring.enqueue(marked(n)).expect("room again once the controller ran them");
    }
}

#[test]
fn a_command_ring_refuses_a_command_past_those_the_controller_has_not_run() {
    let mut ring = CommandRing::new(&DmaPool::new(9, 1)).expect("ring");
    let first = ring.enqueue(marked(0)).expect("first");
    for n in 1..ROOM {
        ring.enqueue(marked(n)).expect("room");
    }
    assert_eq!(ring.enqueue(marked(ROOM)), Err(XhciError::CommandRingFull));
    assert_eq!(marker_at(first), marked(0).d0, "the first command is still there");
    ring.retire();
    ring.enqueue(marked(ROOM)).expect("room again");
}
