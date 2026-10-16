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

//! An interrupt-IN report is armed at the endpoint's whole payload, so a
//! device that sends a full packet never babbles, whatever the client asks
//! for; the client gets at most what it asked. Reports were capped at 8
//! bytes: a longer packet halted the endpoint and was lost.

use nonos_libc::dma_host;

use super::bench::Bench;
use super::hc_device::lock;
use crate::event_ring::events::{transfer, CC_SUCCESS};
use crate::protocol::{E_AGAIN, E_INVAL, OP_ALLOC_TRANSFER_RING, OP_INTERRUPT_IN};
use crate::trb::read_volatile_at;

const HIGH_SPEED: u32 = 3;

/// The length of the TRB armed on the slot's interrupt endpoint at `dci`.
fn armed_length(b: &mut Bench, slot: u8) -> (u64, u32) {
    let res = b.mux.ctxs[0].driver.slots.resources_mut(slot, 8).expect("addressed");
    let phys = res.interrupt[0].armed.expect("armed");
    // SAFETY: `dma_host` names the live ring behind the armed TRB.
    (phys, unsafe { read_volatile_at(dma_host(phys).expect("ring")) }.d2 & 0x1_FFFF)
}

#[test]
fn a_report_is_armed_at_the_endpoints_payload_and_handed_up_to_the_request() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    assert_eq!(b.call(OP_ALLOC_TRANSFER_RING, &[slot, 0x81, 64, 0, 4, 0]).status, 0);
    assert_eq!(b.call(OP_INTERRUPT_IN, &[slot, 3, 8, 0]).status, E_AGAIN);
    let (phys, len) = armed_length(&mut b, slot);
    assert_eq!(len, 64, "a full 64-byte packet fits");
    lock(&b.hc).producer.post(transfer(phys, CC_SUCCESS, 64 - 20, slot, 3));
    let r = b.call(OP_INTERRUPT_IN, &[slot, 3, 8, 0]);
    assert_eq!((r.status, r.body[0], r.body.len()), (0, 8, 2 + 8), "8 of the 20 sent");
}

#[test]
fn a_high_bandwidth_endpoint_gets_its_three_packets_and_nothing_wild_is_configured() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let three = (1024u16 | 2 << 11).to_le_bytes();
    for wild in [[0x81, 0, 0], [0x81, 0x00, 0x1C], [0x01, 64, 0], [0x80, 64, 0], [0x81, 0xFF, 0x07]]
    {
        let body = [slot, wild[0], wild[1], wild[2], 4, 0];
        assert_eq!(b.call(OP_ALLOC_TRANSFER_RING, &body).status, E_INVAL, "{wild:x?}");
    }
    assert_eq!(b.call(OP_ALLOC_TRANSFER_RING, &[slot, 0x81, three[0], three[1], 4, 0]).status, 0);
    assert_eq!(b.call(OP_INTERRUPT_IN, &[slot, 3, 0x00, 0x0C]).status, E_AGAIN, "3072 asked");
    assert_eq!(armed_length(&mut b, slot).1, 3072);
    for wild in [[0u8, 0u8], [0x01, 0x0C]] {
        assert_eq!(b.call(OP_INTERRUPT_IN, &[slot, 3, wild[0], wild[1]]).status, E_INVAL);
    }
}
