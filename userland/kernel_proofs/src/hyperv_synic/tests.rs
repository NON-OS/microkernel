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

//! The partition's offer, the MSR values, the hypercall words, a slot and
//! the ring.

use super::features::{Features, LEAF_FEATURES};
use super::hypercall_abi::*;
use super::message::{read, Slot, MESSAGE_BYTES, PAYLOAD_MAX};
use super::regs::*;
use super::ring::{Ring, RING_SLOTS};

#[test]
fn the_synic_is_used_only_with_every_grant() {
    let all = Features { eax: 1 << 2 | 1 << 5 | 1 << 6, ebx: 1 << 4 | 1 << 5 };
    assert!(all.synic_usable());
    for bit in [2, 5, 6] {
        assert!(!Features { eax: all.eax & !(1 << bit), ..all }.synic_usable(), "eax {bit}");
    }
    for bit in [4, 5] {
        assert!(!Features { ebx: all.ebx & !(1 << bit), ..all }.synic_usable(), "ebx {bit}");
    }
    assert!(!Features::default().synic_usable());
    assert_eq!(LEAF_FEATURES, 0x4000_0003);
}

#[test]
fn the_msrs_and_their_values() {
    assert_eq!(
        [MSR_GUEST_OS_ID, MSR_HYPERCALL, MSR_VP_INDEX, MSR_SCONTROL, MSR_SIEFP, MSR_SIMP],
        [0x4000_0000, 0x4000_0001, 0x4000_0002, 0x4000_0080, 0x4000_0082, 0x4000_0083]
    );
    assert_eq!((MSR_EOM, MSR_SINT0 + SINT_VMBUS), (0x4000_0084, 0x4000_0092));
    assert_eq!(page(0x1234_5678), 0x1234_5001, "the page's address, enabled");
    assert_eq!(sint(VECTOR_HV_SYNIC, true), 0x1_00C1, "masked");
    assert_eq!(sint(VECTOR_HV_SYNIC, false), 0xC1);
    assert_eq!(scontrol_enabled(), 1);
    assert_eq!(GUEST_OS_ID, 1 << 63, "an open-source guest, and nothing more");
}

#[test]
fn the_vector_is_past_the_broker_pool_and_free_of_gates() {
    // The broker's pool ends at 0xC0; 0xC1 to 0xF9 have no gate.
    assert!((0xC1..=0xF9).contains(&VECTOR_HV_SYNIC));
}

#[test]
fn hypercall_words() {
    assert_eq!(control(CALL_POST_MESSAGE, false), 0x5C);
    assert_eq!(control(CALL_SIGNAL_EVENT, true), 0x1_005D);
    assert_eq!(signal_input(0x1234, 0), 0x1234);
    assert_eq!(signal_input(7, 3), 3 << 32 | 7);
    assert_eq!(status(0xDEAD_0000_0013), STATUS_INSUFFICIENT_BUFFERS);
    assert_eq!(status(0x1_0000_0000), STATUS_SUCCESS);
}

#[test]
fn a_post_page_is_its_header_then_the_payload() {
    let mut page = [0xAAu8; 4096];
    post_input(&mut page, 4, 1, &[1, 2, 3]).unwrap();
    assert_eq!(&page[..19], &[4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 2, 3]);
    assert!(post_input(&mut page, 4, 1, &[0; PAYLOAD_MAX]).is_some());
    assert!(post_input(&mut page, 4, 1, &[0; PAYLOAD_MAX + 1]).is_none(), "past a slot");
    assert!(post_input(&mut page, 4, 0, &[1]).is_none(), "type 0 is no message");
    assert!(post_input(&mut [0u8; 8], 4, 1, &[1]).is_none(), "a page too short");
}

fn slot(ty: u32, size: u8, flags: u8) -> [u8; MESSAGE_BYTES] {
    let mut m = [0u8; MESSAGE_BYTES];
    m[0..4].copy_from_slice(&ty.to_le_bytes());
    m[4] = size;
    m[5] = flags;
    m
}

#[test]
fn a_slot_is_free_or_holds_a_bounded_message() {
    assert_eq!(read(&slot(0, 40, 1)), Slot::Free);
    assert_eq!(read(&slot(1, 40, 0)), Slot::Full { ty: 1, len: 40, pending: false });
    assert_eq!(read(&slot(1, 40, 1)), Slot::Full { ty: 1, len: 40, pending: true });
    assert_eq!(
        read(&slot(1, 255, 0)),
        Slot::Full { ty: 1, len: PAYLOAD_MAX, pending: false },
        "a size past the slot is held to it"
    );
}

#[test]
fn the_ring_keeps_order_and_refuses_when_full() {
    let mut r = Ring::new();
    assert!(r.pop().is_none());
    for i in 0..RING_SLOTS {
        assert!(r.push(&slot(i as u32 + 1, 0, 0)));
    }
    assert!(!r.push(&slot(99, 0, 0)), "full: the message stays in the slot");
    for i in 0..RING_SLOTS / 2 {
        assert_eq!(r.pop().unwrap()[0], i as u8 + 1);
    }
    for i in 0..RING_SLOTS / 2 {
        assert!(r.push(&slot(100 + i as u32, 0, 0)), "room again, wrapping");
    }
    let order: Vec<u8> = std::iter::from_fn(|| r.pop()).map(|m| m[0]).collect();
    let want: Vec<u8> = (RING_SLOTS / 2 + 1..=RING_SLOTS)
        .map(|i| i as u8)
        .chain((0..RING_SLOTS / 2).map(|i| 100 + i as u8))
        .collect();
    assert_eq!(order, want);
}
