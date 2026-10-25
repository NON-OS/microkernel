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

//! How a refused seal, a keyring refusal and a full disk are each said.

use super::keep_disk::Disk;
use crate::keep_plan::{keep_records, kept_status, Kept};
use crate::keyring_says::{refusal, unnamed};
use crate::wallet::vault::Unsealed;

#[test]
fn a_seal_refusal_is_named_as_itself() {
    for why in [Unsealed::NoMachineKey, Unsealed::Keyring] {
        let mut disk = Disk::failing(Some("vault"));
        disk.seal = why;
        assert_eq!(keep_records(&mut disk, false), Kept::Lost(why));
    }
    assert_eq!(Unsealed::from_seal(-2), Unsealed::NoMachineKey);
    assert_eq!(Unsealed::from_seal(-11), Unsealed::Keyring);
    assert_eq!(Unsealed::from_seal(-22), Unsealed::Keyring);
}

#[test]
fn each_keyring_refusal_is_its_own_sentence() {
    let invalid: &[u8] = b"bad phrase";
    let said =
        [refusal(-13, invalid), refusal(-28, invalid), refusal(-11, invalid), refusal(-1, invalid)];
    for (i, a) in said.iter().enumerate() {
        assert_ne!(*a, invalid);
        for b in &said[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert_eq!(refusal(-22, invalid), invalid);
    assert_ne!(unnamed(true), unnamed(false));
}

#[test]
fn a_full_disk_is_said_as_one() {
    let mut disk = Disk::failing(Some("words"));
    disk.seal = Unsealed::DiskFull;
    assert_eq!(keep_records(&mut disk, false), Kept::Lost(Unsealed::DiskFull));
    assert!(!disk.log.contains(&"vault"));
    assert!(kept_status(Kept::Lost(Unsealed::DiskFull), false)
        .unwrap()
        .starts_with(b"the disk is full"));
    assert_eq!(Unsealed::from_store(-28), Unsealed::DiskFull);
    assert_eq!(Unsealed::from_store(-5), Unsealed::Store);
}
