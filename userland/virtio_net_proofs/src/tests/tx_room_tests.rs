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

//! Room for net.core's bursts: it drops a frame answered E_AGAIN, so the
//! transmit side holds as many frames as the wired drivers' 32-entry rings
//! before it says so. Every descriptor and buffer still lies inside its
//! grant, and the device is still told a 256-entry ring.

use super::drive::{call, driver, status};
use super::harness::{Dev, TX_BUFS_AT};
use crate::constants::{Q_TX, RING_SLOTS, TX_BUFFER_LEN, TX_DESC_COUNT, VQ_AVAIL_OFFSET};
use crate::program_queue::program_queue;
use crate::protocol::{E_AGAIN, OP_TX_PACKET};
use crate::queue::TxQueue;
use crate::regs::{fresh, Regs};

/// What the wired drivers (e1000, e1000e, igc) keep in flight.
const BURST: u16 = 32;

#[test]
fn a_burst_as_long_as_a_wired_drivers_ring_is_queued_without_e_again() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    for n in 0..BURST {
        let (_, reply) = call(&mut d, OP_TX_PACKET, &[n as u8; 60]);
        assert_eq!(status(&reply.unwrap()), 0, "frame {n} was refused");
    }
    let (_, reply) = call(&mut d, OP_TX_PACKET, &[0; 60]);
    assert_eq!(status(&reply.unwrap()), E_AGAIN, "more frames held than buffers");
}

#[test]
fn every_transmit_descriptor_and_buffer_lies_inside_its_grant() {
    let dev = Dev::tx();
    let mut tx = dev.tx_queue();
    while tx.post(&[0xFF; 1514]).is_ok() {}
    assert_eq!(tx.in_flight.count_ones(), u32::from(TX_DESC_COUNT));
    let grant = u64::from(TX_BUFFER_LEN) * u64::from(TX_DESC_COUNT);
    for slot in 0..usize::from(TX_DESC_COUNT) {
        assert!(16 * slot + 16 <= VQ_AVAIL_OFFSET, "descriptor {slot} past the table");
        let addr = u64::from(dev.ring.u32_at(16 * slot)) - TX_BUFS_AT;
        let len = u64::from(dev.ring.u32_at(16 * slot + 8));
        assert!(addr + len <= grant, "buffer {slot} runs past the grant");
    }
}

#[test]
fn the_device_is_told_256_entries_and_the_driver_uses_its_own_count() {
    fresh(RING_SLOTS);
    let count = program_queue(Regs, Q_TX, 0x20_0000, TxQueue::queue_size());
    assert_eq!(count, Ok(TX_DESC_COUNT));
    let dev = Dev::tx();
    let tx = TxQueue::new(dev.ring.view(), dev.bufs.view(), TX_BUFS_AT, RING_SLOTS);
    assert_eq!(tx.buf_count, TX_DESC_COUNT, "the queue keeps to its buffers");
}
