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

//! W2c: pinning a saved network to WPA3 keeps its passphrase and other flags,
//! changes nothing for a network not saved or already pinned, and the pinned
//! network's joins then carry the WPA3-only flag, which both drivers refuse
//! to run as WPA2 (code -7).

use super::list::SavedList;
use super::pin::pin_in;
use crate::join_wire::{encode_join, JoinFlags, JOIN_BODY_MAX};

const HIDDEN: JoinFlags = JoinFlags { wpa3_only: false, hidden: true };

#[test]
fn a_pin_sets_wpa3_and_keeps_the_passphrase_and_the_hidden_mark() {
    let mut l = SavedList::new();
    assert!(l.put(b"Cafe", b"other-pass"));
    assert!(l.put_with(b"Home", b"password123", HIDDEN));
    assert!(pin_in(&mut l, b"Home"));
    let i = l.find(b"Home").unwrap();
    assert_eq!(l.join_flags(i), JoinFlags { wpa3_only: true, hidden: true });
    assert_eq!(l.passphrase(i), b"password123");
    let cafe = l.find(b"Cafe").unwrap();
    assert_eq!(
        (l.join_flags(cafe), l.passphrase(cafe)),
        (JoinFlags::default(), &b"other-pass"[..])
    );
    assert_eq!(l.len(), 2);
}

#[test]
fn a_network_not_saved_or_already_pinned_is_left_as_it_is() {
    let mut l = SavedList::new();
    assert!(!pin_in(&mut l, b"Home"), "a network not saved stays unsaved");
    assert!(l.is_empty());
    assert!(l.put(b"Home", b"password123"));
    assert!(pin_in(&mut l, b"Home"));
    assert!(!pin_in(&mut l, b"Home"), "pinned once; no second write");
    assert!(!pin_in(&mut l, b""), "an empty name");
}

#[test]
fn only_forget_clears_a_pin_and_a_pinned_join_carries_the_wpa3_only_flag() {
    let mut l = SavedList::new();
    assert!(l.put(b"Home", b"password123"));
    assert!(pin_in(&mut l, b"Home"));
    assert!(l.put_with(b"Home", b"new-password", JoinFlags::default()), "saved again");
    let i = l.find(b"Home").unwrap();
    assert!(l.join_flags(i).wpa3_only, "saving again cannot unpin");
    let mut out = [0u8; JOIN_BODY_MAX];
    let n = encode_join(l.ssid(i), l.passphrase(i), l.join_flags(i), &mut out);
    assert_eq!(out[n - 1] & 1, 1, "flags bit 0, WPA3 only, which both drivers read");
    l.remove(i); // what forget does to the list
    assert!(l.put(b"Home", b"new-password"));
    assert!(!l.join_flags(l.find(b"Home").unwrap()).wpa3_only, "forgotten, then saved anew");
}
