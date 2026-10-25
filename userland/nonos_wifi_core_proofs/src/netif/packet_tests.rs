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

//! The adapter's transmit and receive operations: the Ethernet frame reaches
//! the link, an unassociated link refuses it, and an empty queue is a retry.

use crate::netif::envelope_tests::{parse, request, status};
use crate::netif::link::MockLink;
use nonos_wifi_core::netif::serve::{serve, MAX_RESPONSE};
use nonos_wifi_core::netif::wire::{HDR_LEN, OP_RX_PACKET, OP_TX_PACKET};

#[test]
fn transmit_hands_the_ethernet_frame_to_the_link() {
    let mut link = MockLink::up_with([0; 6]);
    let frame = [0xFFu8; 64]; // a broadcast-ish Ethernet frame
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_TX_PACKET, 9, &frame), &mut link, &mut out).unwrap();
    let (op, rid, plen) = parse(&out[..n]);
    assert_eq!((op, rid, plen), (OP_TX_PACKET, 9, 4));
    assert!(status(&out[..n]) >= 0, "accepted");
    assert_eq!(link.sent.borrow().len(), 1);
    assert_eq!(link.sent.borrow()[0], frame.to_vec(), "the frame reaches the CCMP/tx path");
}

#[test]
fn transmit_while_unassociated_is_refused() {
    let mut link = MockLink::down();
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_TX_PACKET, 4, &[0u8; 60]), &mut link, &mut out).unwrap();
    assert!(status(&out[..n]) < 0, "net_core sees the transmit fail");
    assert!(link.sent.borrow().is_empty());
}

#[test]
fn receive_returns_a_queued_ethernet_frame_then_reports_empty() {
    let mut link = MockLink::up_with([0; 6]);
    let frame = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 1, 2, 3, 4, 5, 6, 0x08, 0x00, 0x45];
    link.rx.borrow_mut().push(frame.clone());
    let mut out = [0u8; MAX_RESPONSE];

    let n = serve(&request(OP_RX_PACKET, 5, &[]), &mut link, &mut out).unwrap();
    let (op, rid, plen) = parse(&out[..n]);
    assert_eq!((op, rid), (OP_RX_PACKET, 5));
    assert_eq!(plen as usize, 4 + 4 + frame.len());
    assert_eq!(status(&out[..n]), 0);
    let flen = u32::from_le_bytes([
        out[HDR_LEN + 4],
        out[HDR_LEN + 5],
        out[HDR_LEN + 6],
        out[HDR_LEN + 7],
    ]) as usize;
    assert_eq!(flen, frame.len());
    assert_eq!(&out[HDR_LEN + 8..HDR_LEN + 8 + flen], &frame[..]);

    // The queue is now empty: the next poll is a retry status, no frame.
    let n = serve(&request(OP_RX_PACKET, 6, &[]), &mut link, &mut out).unwrap();
    let (_, _, plen) = parse(&out[..n]);
    assert_eq!(plen, 4);
    assert_ne!(status(&out[..n]), 0, "empty poll is a retry, not a zero-length frame");
}
