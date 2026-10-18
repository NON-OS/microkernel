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
//! The camera wire's frames, OP_OPEN's listing bytes, and a collected
//! batch read into payloads.

use crate::batch::payloads;
use crate::listing::listing;
use crate::webcam::config;
use crate::wire::{decode, reply, MAGIC, OP_OPEN};

#[test]
fn a_request_reads_and_a_reply_is_framed() {
    let mut msg = vec![0u8; 24];
    msg[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    msg[4..6].copy_from_slice(&1u16.to_le_bytes());
    msg[6..8].copy_from_slice(&OP_OPEN.to_le_bytes());
    msg[12..16].copy_from_slice(&9u32.to_le_bytes());
    msg[16..20].copy_from_slice(&4u32.to_le_bytes());
    let (req, body) = decode(&msg).expect("a request");
    assert_eq!((req.op, req.id, body.len()), (OP_OPEN, 9, 4));
    let mut tx = [0u8; 40];
    assert_eq!(reply(&mut tx, &req, -1, &[]), 24);
    assert_eq!(i32::from_le_bytes(tx[20..24].try_into().unwrap()), -1);
    msg[16..20].copy_from_slice(&5u32.to_le_bytes());
    assert!(decode(&msg).is_none(), "a body past the message");
    msg[0] ^= 1;
    assert!(decode(&msg).is_none(), "another magic");
}

#[test]
fn the_listing_names_every_format_and_frame_that_fits() {
    let cam = nonos_uvc::camera(&config()).unwrap();
    let l = listing(&cam, (0x046D, 0x0825), 4000);
    assert_eq!(&l[..5], &[0x6D, 0x04, 0x25, 0x08, 2]);
    assert_eq!(&l[5..12], &[1, 1, b'Y', b'U', b'Y', b'2', 2]);
    assert_eq!(&l[12..21], &[1, 0x80, 0x02, 0xE0, 0x01, 0x15, 0x16, 0x05, 0]);
    assert_eq!(l.len(), 5 + (7 + 18) + (7 + 9));
    assert_eq!(listing(&cam, (0, 0), 40)[4], 1, "only what fits");
}

#[test]
fn a_batch_reads_into_its_payloads_and_no_further_than_it_carries() {
    let mut data = vec![1, 3, 0, 0];
    let mut lens = vec![0u8; 62];
    lens[0..2].copy_from_slice(&2u16.to_le_bytes());
    lens[4..6].copy_from_slice(&3u16.to_le_bytes());
    data.extend(lens);
    data.extend([10, 11, 20, 21, 22]);
    let got: Vec<&[u8]> = payloads(&data).collect();
    assert_eq!(got, vec![&[10u8, 11][..], &[][..], &[20, 21, 22][..]]);
    data.truncate(data.len() - 1);
    assert_eq!(payloads(&data).count(), 2, "the third is cut short");
}
