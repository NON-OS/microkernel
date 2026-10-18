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

//! Transmit without waiting: a frame goes into a free buffer behind a zeroed
//! header, padded to the Ethernet minimum, and is offered; with every buffer
//! the device's the answer is Full at once, never a spin.

use super::harness::{Dev, TX_BUFS_AT};
use super::mem::Mem;
use crate::constants::{MAX_ETHERNET_FRAME, TX_BUFFER_LEN, TX_DESC_COUNT, VIRTIO_NET_HDR_LEN};
use crate::queue::TxError;

const HDR: usize = VIRTIO_NET_HDR_LEN;

/// Descriptor `i`: address, length, flags.
fn desc(ring: &Mem, i: usize) -> (u64, u32, u16) {
    let addr = u64::from(ring.u32_at(16 * i)) | u64::from(ring.u32_at(16 * i + 4)) << 32;
    (addr, ring.u32_at(16 * i + 8), ring.u16_at(16 * i + 12))
}

#[test]
fn a_frame_is_queued_padded_behind_a_zeroed_header() {
    let dev = Dev::tx();
    dev.bufs.put(0, &[0xEE; 100]);
    let mut tx = dev.tx_queue();
    assert_eq!(tx.post(&[0xAB; 20]), Ok(()));
    assert_eq!(desc(&dev.ring, 0), (TX_BUFS_AT, (HDR + 60) as u32, 0));
    assert!((0..HDR).all(|i| dev.bufs.get(i) == 0), "a header byte is not zero");
    assert!((HDR..HDR + 20).all(|i| dev.bufs.get(i) == 0xAB));
    assert!((HDR + 20..HDR + 60).all(|i| dev.bufs.get(i) == 0), "the padding is not zero");
    assert_eq!((dev.avail_idx(), dev.avail(0), tx.in_flight), (1, 0, 1));
}

#[test]
fn a_full_ring_answers_full_at_once_and_a_completion_frees_its_buffer() {
    let dev = Dev::tx();
    let mut tx = dev.tx_queue();
    for slot in 0..TX_DESC_COUNT {
        assert_eq!(tx.post(&[slot as u8; 64]), Ok(()));
        let addr = TX_BUFS_AT + u64::from(slot) * u64::from(TX_BUFFER_LEN);
        assert_eq!(desc(&dev.ring, slot.into()).0, addr);
    }
    assert_eq!(tx.post(&[1; 64]), Err(TxError::Full));
    dev.complete(0, 5, 0);
    assert_eq!(tx.post(&[0x99; 64]), Ok(()), "the buffer that came back is used");
    assert_eq!(dev.avail(TX_DESC_COUNT), 5);
    assert_eq!(dev.bufs.get(5 * TX_BUFFER_LEN as usize + HDR), 0x99);
}

#[test]
fn a_frame_no_buffer_can_hold_is_refused() {
    let dev = Dev::tx();
    let mut tx = dev.tx_queue();
    assert_eq!(tx.post(&vec![0; MAX_ETHERNET_FRAME + 1]), Err(TxError::TooLong));
    assert_eq!(tx.with_hdr_len(2048).post(&[0; 60]), Err(TxError::TooLong));
    assert_eq!(tx.with_hdr_len(usize::MAX).post(&[0; 60]), Err(TxError::TooLong));
    assert_eq!(tx.post(&vec![0; MAX_ETHERNET_FRAME]), Ok(()));
    assert_eq!(dev.avail_idx(), 1, "only the frame that fits was offered");
}
