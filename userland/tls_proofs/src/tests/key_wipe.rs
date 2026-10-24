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

//! A connection's keys leave nothing behind: every copy of a TrafficKeys,
//! the one a stream holds and each one a caller cloned, is zeros where it
//! lay once it is dropped.

use core::mem::ManuallyDrop;
use core::ptr;

use crate::traffic_keys::TrafficKeys;

fn keys() -> TrafficKeys {
    TrafficKeys {
        suite: 0x1303,
        handshake_secret: [1; 32],
        client_secret: [2; 32],
        server_secret: [3; 32],
        client_key: [4; 32],
        client_iv: [5; 12],
        server_key: [6; 32],
        server_iv: [7; 12],
    }
}

fn bytes_of(k: &TrafficKeys) -> (*const u8, usize) {
    (k as *const TrafficKeys as *const u8, core::mem::size_of::<TrafficKeys>())
}

/// Every secret byte of a dropped value, read where it lay; the suite is not
/// a secret and is left out.
fn secrets_after_drop(mut k: ManuallyDrop<TrafficKeys>) -> Vec<u8> {
    let fields = [
        (k.handshake_secret.as_ptr(), 32),
        (k.client_secret.as_ptr(), 32),
        (k.server_secret.as_ptr(), 32),
        (k.client_key.as_ptr(), 32),
        (k.client_iv.as_ptr(), 12),
        (k.server_key.as_ptr(), 32),
        (k.server_iv.as_ptr(), 12),
    ];
    let (_, size) = bytes_of(&k);
    assert!(size >= 184, "every field is inside the value");
    // SAFETY: dropped once; the storage is only read afterwards.
    unsafe { ManuallyDrop::drop(&mut k) };
    fields
        .iter()
        // SAFETY: each pointer is into storage `k` still owns.
        .flat_map(|&(p, n)| (0..n).map(move |i| unsafe { ptr::read_volatile(p.add(i)) }))
        .collect()
}

#[test]
fn a_dropped_set_of_keys_is_zeros_where_it_lay() {
    let got = secrets_after_drop(ManuallyDrop::new(keys()));
    assert_eq!(got.len(), 184);
    assert!(got.iter().all(|b| *b == 0), "{got:?}");
}

#[test]
fn a_clone_is_wiped_on_its_own_drop_and_the_original_is_untouched() {
    let original = keys();
    let copy = ManuallyDrop::new(original.clone());
    let got = secrets_after_drop(copy);
    assert!(got.iter().all(|b| *b == 0));
    assert_eq!(original.client_key, [4; 32], "the kept one still works");
}
