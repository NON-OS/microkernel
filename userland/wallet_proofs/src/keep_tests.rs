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

//! A new wallet's records: the vault cleared first and written last, each
//! write checked, and the status naming what the next boot will find.

use super::keep_disk::Disk;
use crate::keep_plan::{keep_records, kept_status, Kept};
use crate::wallet::vault::Unsealed;

#[test]
fn a_wallet_with_words_writes_the_vault_last() {
    let mut disk = Disk::failing(None);
    assert_eq!(keep_records(&mut disk, false), Kept::Landed);
    assert_eq!(disk.log, ["clear vault", "kind", "words", "accounts", "vault"]);
    assert_eq!(kept_status(Kept::Landed, false), None);
}

#[test]
fn a_wallet_from_a_key_clears_the_last_words() {
    let mut disk = Disk::failing(None);
    assert_eq!(keep_records(&mut disk, true), Kept::Landed);
    assert_eq!(disk.log, ["clear vault", "kind", "clear words", "accounts", "vault"]);
}

#[test]
fn a_vault_that_will_not_clear_stops_everything() {
    let mut disk = Disk::failing(Some("clear vault"));
    assert_eq!(keep_records(&mut disk, false), Kept::LastStays(Unsealed::Store));
    assert_eq!(disk.log, ["clear vault"]);
    let said = kept_status(Kept::LastStays(Unsealed::Store), false).unwrap();
    assert!(said.windows(31).any(|w| w == b"any wallet stored before stays:"));
}

#[test]
fn a_failure_after_the_clear_writes_no_vault() {
    for at in ["kind", "words", "accounts"] {
        let mut disk = Disk::failing(Some(at));
        assert_eq!(keep_records(&mut disk, false), Kept::Lost(Unsealed::Store), "{at}");
        assert!(!disk.log.contains(&"vault"), "{at}");
        assert_eq!(disk.log.last(), Some(&at));
    }
    let mut disk = Disk::failing(Some("clear words"));
    assert_eq!(keep_records(&mut disk, true), Kept::Lost(Unsealed::Store));
    assert!(!disk.log.contains(&"vault"));
}
