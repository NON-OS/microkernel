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

//! net.nym's secrets leave nothing behind when they are done: a dropped copy
//! of the client identity, and a gateway key cleared on disconnect or
//! replaced by the next one. The bytes are read back where they lay.

use core::mem::ManuallyDrop;
use core::ptr;

use crate::state::identity::{client_identity, Identity};
use crate::state::shared_key::{gateway_shared_key, set_gateway_shared_key, wipe_slot, Slot};

fn read_back(p: *const u8) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        // SAFETY: `p` points into a value the test still owns the storage of.
        *b = unsafe { ptr::read_volatile(p.add(i)) };
    }
    out
}

#[test]
fn a_dropped_identity_leaves_no_seed_where_it_lay() {
    let mut id = ManuallyDrop::new(Identity { seed: [0xA5; 32], public: [1; 32] });
    let at = id.seed.as_ptr();
    // SAFETY: dropped once, and the storage is read only after, never used.
    unsafe { ManuallyDrop::drop(&mut id) };
    assert_eq!(read_back(at), [0u8; 32]);
}

#[test]
fn every_copy_the_state_hands_out_is_wiped_when_it_is_dropped() {
    let first = client_identity().expect("an identity is drawn");
    let mut copy = ManuallyDrop::new(client_identity().expect("the same one"));
    assert_eq!(copy.seed, first.seed, "one identity for the boot");
    let at = copy.seed.as_ptr();
    // SAFETY: as above.
    unsafe { ManuallyDrop::drop(&mut copy) };
    assert_eq!(read_back(at), [0u8; 32]);
    assert_eq!(client_identity().expect("still held").seed, first.seed, "the held one survives");
}

#[test]
fn a_cleared_gateway_key_is_zeros_where_it_lay() {
    let mut slot = Slot { key: [0x3C; 32], set: true };
    let at = slot.key.as_ptr();
    wipe_slot(&mut slot);
    assert!(!slot.set);
    assert_eq!(read_back(at), [0u8; 32]);
}

#[test]
fn the_next_gateway_key_replaces_the_last_and_clearing_forgets_it() {
    set_gateway_shared_key(&[7; 32]);
    set_gateway_shared_key(&[9; 32]);
    assert_eq!(gateway_shared_key(), Some([9; 32]));
    crate::state::shared_key::clear_gateway_shared_key();
    assert_eq!(gateway_shared_key(), None);
}
