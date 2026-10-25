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

//! E-37c: the PMKSA and the temporal key are neither Copy nor Clone, and
//! each wipes its key when dropped. The bytes are read back where the key
//! was, after its Drop has run but before the storage is reused.

use core::mem::{needs_drop, ManuallyDrop};
use nonos_wifi_core::pmksa::Pmksa;
use nonos_wifi_core::rsn::TemporalKey;

/// The `n` octets at `at` once `v`'s Drop has run in place.
fn after_drop<T>(v: T, at: impl FnOnce(&T) -> (*const u8, usize)) -> Vec<u8> {
    let mut v = ManuallyDrop::new(v);
    let (p, n) = at(&v);
    // SAFETY: `v` is dropped once and its storage stays in place, so `p`
    // still points at `n` initialised octets that Drop wrote over.
    unsafe {
        ManuallyDrop::drop(&mut v);
        core::slice::from_raw_parts(p, n).to_vec()
    }
}

#[test]
fn both_key_types_need_drop() {
    assert!(needs_drop::<Pmksa>() && needs_drop::<TemporalKey>());
}

#[test]
fn a_dropped_pmksa_leaves_no_pmk_or_pmkid() {
    let sa = Pmksa { bssid: [2, 0, 0, 0, 0, 1], pmk: [0xa5; 32], pmkid: [0x5a; 16] };
    assert_eq!(after_drop(sa, |s| (s.pmk.as_ptr(), 32)), [0; 32]);
    let sa = Pmksa { bssid: [2, 0, 0, 0, 0, 1], pmk: [0xa5; 32], pmkid: [0x5a; 16] };
    assert_eq!(after_drop(sa, |s| (s.pmkid.as_ptr(), 16)), [0; 16]);
}

#[test]
fn a_dropped_temporal_key_leaves_no_key() {
    let at = |k: &TemporalKey| (k.bytes().as_ptr(), k.bytes().len());
    assert_eq!(after_drop(TemporalKey::Ccmp128([0xc3; 16]), at), [0; 16]);
    assert_eq!(after_drop(TemporalKey::Gcmp256([0x3c; 32]), at), [0; 32]);
}
