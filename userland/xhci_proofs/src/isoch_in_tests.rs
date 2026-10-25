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

//! Isochronous IN: an odd DCI typed Isoch IN, the batches an IN request
//! may ask for, each TD's bytes from its event, and the packets packed.

use crate::constants::{CC_MISSED_SERVICE, CC_SHORT_PACKET, CC_SUCCESS, CC_USB_TRANSACTION};
use crate::controller::{in_batch_fits, isoch_endpoint, lengths, pack, td_bytes};
use crate::controller::{write_isoch_input, IsochEndpoint};
use crate::dma::{DmaPool, DmaRegion};

fn dw(r: &DmaRegion, context: usize, dword: usize) -> u32 {
    let at = context * 32 + dword * 4;
    // SAFETY: the region holds 33 contexts of 32 bytes.
    unsafe { core::ptr::read_volatile(r.as_mut_ptr::<u8>().add(at) as *const u32) }
}

/// DCI 5 is endpoint 2 IN: the same context, typed Isoch IN.
#[test]
fn an_odd_dci_is_an_isoch_in_endpoint() {
    let pool = DmaPool::new(1, 1);
    let (input, output) = (pool.alloc(33 * 32).unwrap(), pool.alloc(32 * 32).unwrap());
    output.zero();
    let ep = IsochEndpoint { ring_phys: 0x3000, ..isoch_endpoint(1, false, 5, 1, (0, 0), 200) };
    write_isoch_input(&input, &output, 32, ep);
    assert_eq!(dw(&input, 0, 1), 1 | (1 << 5), "the slot and DCI 5 added");
    let dw1 = dw(&input, 6, 1);
    assert_eq!((dw1 >> 3) & 0x7, 5, "Isoch IN");
    assert_eq!(((dw1 >> 8) & 0xFF, dw1 >> 16), (0, 200), "Max Burst, Max Packet Size");
}

#[test]
fn an_in_batch_fits_one_reply() {
    assert!(in_batch_fits(20, 200), "20 full-speed intervals of a 48 kHz mic");
    assert!(!in_batch_fits(0, 200), "no packet");
    assert!(!in_batch_fits(32, 64), "past 31 packets");
    assert!(!in_batch_fits(2, 3072), "6 KiB is past one reply");
    assert!(!in_batch_fits(1, 0), "an endpoint with no payload");
}

#[test]
fn each_td_brings_its_length_or_none() {
    assert_eq!(td_bytes(CC_SUCCESS, 0, 192), Some(192));
    assert_eq!(td_bytes(CC_SHORT_PACKET, 16, 192), Some(176));
    assert_eq!(td_bytes(CC_SHORT_PACKET, 500, 192), Some(0), "residual clamped");
    assert_eq!(td_bytes(CC_MISSED_SERVICE, 192, 192), Some(0));
    assert_eq!(td_bytes(CC_USB_TRANSACTION, 0, 192), Some(0));
    assert_eq!(td_bytes(6, 0, 192), None, "a stall fails the endpoint");
}

#[test]
fn lengths_and_packing_keep_packet_order() {
    let got = [(CC_SUCCESS, 0), (CC_MISSED_SERVICE, 4), (CC_SHORT_PACKET, 1)];
    let mut lens = [0u16; 3];
    assert_eq!(lengths(&got, 4, &mut lens), Ok(CC_MISSED_SERVICE));
    assert_eq!(lens, [4, 0, 3]);
    let half = [1, 2, 3, 4, 9, 9, 9, 9, 5, 6, 7, 0];
    let mut out = [0u8; 12];
    assert_eq!(pack(&half, 4, &lens, &mut out), 7);
    assert_eq!(&out[..7], &[1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(lengths(&[(CC_SUCCESS, 0), (6, 0)], 4, &mut lens), Err(6));
}
