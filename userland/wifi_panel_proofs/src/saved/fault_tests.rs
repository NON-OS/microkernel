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

//! The sealed saved-networks record under corruption. The record is
//! ChaCha20-Poly1305 over the slots, with the magic and path as associated
//! data (`file`), so any change to it fails to open: a corrupt file yields
//! "none saved", never a saved policy, so never one weaker than was written.
//! The authentic record opens to exactly the policy written, WPA3 included.
//! Proved beside the bond-store fault proofs in spirit: a damaged store never
//! downgrades what it holds.

use super::file::{is_withdrawn, open_file, seal_file, FILE_LEN};
use super::list::SavedList;
use crate::join_wire::JoinFlags;

const KEY: [u8; 32] = [7u8; 32];
const NONCE: [u8; 12] = [9u8; 12];
const WPA3: JoinFlags = JoinFlags { wpa3_only: true, hidden: false };

fn sealed_wpa3() -> [u8; FILE_LEN] {
    let mut list = SavedList::new();
    assert!(list.put_with(b"Home", b"a-good-passphrase", WPA3));
    let mut file = [0u8; FILE_LEN];
    assert!(seal_file(&list, &KEY, &NONCE, &mut file), "sealed");
    file
}

#[test]
fn the_authentic_record_opens_to_the_policy_as_written() {
    let good = sealed_wpa3();
    let list = open_file(&good, &KEY).ok().expect("opens under its key");
    assert_eq!(list.len(), 1);
    assert_eq!(list.ssid(0), b"Home");
    assert_eq!(list.join_flags(0), WPA3, "the WPA3 policy is exactly as written");
    assert_eq!(list.passphrase(0), b"a-good-passphrase");
    assert!(!is_withdrawn(&good), "a real record is not the withdrawn one");
}

#[test]
fn no_single_byte_corruption_yields_any_policy() {
    let good = sealed_wpa3();
    for i in 0..FILE_LEN {
        for xor in [0x01u8, 0x80, 0xFF] {
            let mut bad = good;
            bad[i] ^= xor;
            assert!(open_file(&bad, &KEY).is_err(), "a flip at byte {i} opened a record");
        }
    }
}

#[test]
fn a_wrong_length_or_wrong_key_opens_nothing() {
    let good = sealed_wpa3();
    assert!(open_file(&good[..FILE_LEN - 1], &KEY).is_err(), "truncated");
    let mut longer = good.to_vec();
    longer.push(0);
    assert!(open_file(&longer, &KEY).is_err(), "over-long");
    assert!(open_file(&good, &[8u8; 32]).is_err(), "another machine's key");
    assert!(is_withdrawn(&[0u8; FILE_LEN]), "the all-zero record is the withdrawn one");
}
