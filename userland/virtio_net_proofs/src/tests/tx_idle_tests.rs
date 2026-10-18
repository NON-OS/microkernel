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

//! A frame the device has sent does not wait for the next send to be wiped
//! (plan/50 C8): net.core polls for received frames every cycle, and each
//! poll takes back, and wipes, every transmit buffer the device returned.
//! A completion the device makes up still frees nothing.

use super::drive::{call, driver, status};
use super::harness::Dev;
use crate::constants::TX_BUFFER_LEN;
use crate::protocol::{E_AGAIN, OP_RX_BATCH, OP_RX_PACKET};
use crate::tx::send;

const POLLS: [(u16, &[u8]); 3] =
    [(OP_RX_PACKET, &[]), (OP_RX_BATCH, &[1, 0, 0, 0]), (OP_RX_BATCH, &[])];

fn wiped(tx: &Dev, slot: usize) -> bool {
    let len = TX_BUFFER_LEN as usize;
    (slot * len..(slot + 1) * len).all(|i| tx.bufs.get(i) == 0)
}

#[test]
fn a_sent_frame_is_wiped_at_the_next_receive_poll() {
    for (op, body) in POLLS {
        let (rx, tx) = (Dev::rx(), Dev::tx());
        let mut d = driver(&rx, &tx);
        assert_eq!(send(&mut d, &[0xAB; 60]), Ok(()));
        tx.complete(0, 0, 0);
        let (_, reply) = call(&mut d, op, body);
        assert!(body.is_empty() || status(&reply.unwrap()) == E_AGAIN, "op {op}");
        assert_eq!(d.tx.in_flight, 0, "op {op}: the buffer was not taken back");
        assert!(wiped(&tx, 0), "op {op}: the sent frame stayed in DMA memory");
    }
}

#[test]
fn a_poll_that_also_hands_up_a_frame_takes_the_buffers_back_too() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    send(&mut d, &[0x11; 60]).unwrap();
    send(&mut d, &[0x22; 60]).unwrap();
    tx.complete(0, 1, 0);
    rx.complete(0, 0, 10 + 60);
    let (_, reply) = call(&mut d, OP_RX_PACKET, &[]);
    assert_eq!(status(&reply.unwrap()), 0, "the received frame was handed up");
    assert_eq!(d.tx.in_flight, 0b01, "only the returned buffer came back");
    assert!(wiped(&tx, 1) && !wiped(&tx, 0));
}

#[test]
fn a_made_up_completion_seen_at_a_poll_frees_nothing() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    send(&mut d, &[0x33; 60]).unwrap();
    for (n, id) in [5u32, u32::MAX, 0x1_0000].into_iter().enumerate() {
        tx.complete(n as u16, id, u32::MAX);
    }
    call(&mut d, OP_RX_PACKET, &[]);
    assert_eq!(d.tx.in_flight, 0b1, "buffer 0 is still the device's");
    assert!(!wiped(&tx, 0), "a buffer the device still reads was wiped");
}
