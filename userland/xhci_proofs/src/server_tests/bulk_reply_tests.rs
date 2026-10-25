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

//! A bulk reply is `moved_le16, 0, 0`, and only a bulk IN follows it with
//! the bytes moved. A bulk OUT reply once carried `moved` bytes more, so the
//! 31-byte CBW's reply was 59 bytes: driver.usb_msc0 gives an OUT reply 28
//! (`DATA_AT + 4`), saw its payload run past what came, took E_IO, and
//! reset the transport after every command it sent.

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{OP_BULK_IN, OP_BULK_OUT, OP_CONFIGURE_BULK};

const SUPER_SPEED: u32 = 4;
/// What driver.usb_msc0 receives a bulk OUT reply into: the header, the
/// status and the four-byte prefix (its `xhci/bulk.rs`).
const OUT_REPLY_BUFFER: usize = 20 + 4 + 4;

fn configured() -> (Bench, u8) {
    let mut b = Bench::new(SUPER_SPEED);
    let slot = b.root_device();
    let pair = [slot, 0x81, 0x02, 0, 0, 4, 0, 4];
    assert_eq!(b.call(OP_CONFIGURE_BULK, &pair).status, 0, "bulk pair 1 IN, 2 OUT, 1024 bytes");
    (b, slot)
}

#[test]
fn a_bulk_out_reply_is_its_count_alone_and_fits_the_class_drivers_buffer() {
    let (mut b, slot) = configured();
    let cbw: Vec<u8> = (0..31).collect();
    let mut body = vec![slot, 0, 31, 0];
    body.extend_from_slice(&cbw);
    let r = b.call(OP_BULK_OUT, &body);
    assert_eq!(r.status, 0);
    assert_eq!(r.body, [31, 0, 0, 0], "the count, and no bytes after it");
    assert!(20 + 4 + r.body.len() <= OUT_REPLY_BUFFER, "a whole reply in usb_msc0's buffer");
    assert_eq!(lock(&b.hc).bulk.out, [cbw], "the device took the CBW as sent");
}

#[test]
fn a_bulk_in_reply_carries_the_bytes_that_came() {
    let (mut b, slot) = configured();
    let inquiry: Vec<u8> = (0..36).map(|i| 0xA0 ^ i).collect();
    lock(&b.hc).bulk.answer = inquiry.clone();
    let r = b.call(OP_BULK_IN, &[slot, 0, 36, 0]);
    assert_eq!(r.status, 0);
    assert_eq!(r.body[..4], [36, 0, 0, 0]);
    assert_eq!(r.body[4..], inquiry[..], "the 36 bytes, and only those");
    lock(&b.hc).bulk.answer = vec![0x55; 13];
    let r = b.call(OP_BULK_IN, &[slot, 0, 64, 0]);
    assert_eq!((r.body[..4].to_vec(), r.body.len()), (vec![13, 0, 0, 0], 4 + 13), "a short packet");
}
