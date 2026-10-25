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

//! An older table or an older list cannot be swapped back in. The host
//! passes the highest table serial it has installed and the newest list
//! issue time it has accepted; anything older is refused, the same is not.
//! The labels both are signed under are pinned here byte for byte.

use crate::build::revoke::RevB;
use crate::build::table::{TableB, SERIAL};
use nsh_crypto::Label;
use nsh_roles::{open_policy, open_revocations, PolicyError};

fn table(min_serial: u64) -> Result<u64, PolicyError> {
    let t = TableB::default();
    open_policy(&t.bytes(), &t.anchor, min_serial).map(|p| p.serial)
}

#[test]
fn a_table_older_than_the_installed_one_is_refused() {
    assert_eq!(table(SERIAL - 1), Ok(SERIAL));
    assert_eq!(table(SERIAL), Ok(SERIAL), "the same table may be reloaded");
    assert_eq!(table(SERIAL + 1), Err(PolicyError::Replayed));
    assert_eq!(table(u64::MAX), Err(PolicyError::Replayed));
}

#[test]
fn a_table_carries_its_serial_and_issue_time() {
    let t = TableB { serial: 77, issued_ms: 1234, ..TableB::default() };
    let bytes = t.bytes();
    let p = open_policy(&bytes, &t.anchor, 77).expect("opens");
    assert_eq!((p.serial, p.issued_ms), (77, 1234));
}

fn list(min_issued_ms: u64) -> Result<u64, PolicyError> {
    let t = TableB::default();
    let tb = t.bytes();
    let policy = open_policy(&tb, &t.anchor, 0).expect("table");
    let r = RevB::default();
    open_revocations(&r.bytes(), &policy, min_issued_ms).map(|l| l.list().issued_ms)
}

#[test]
fn a_list_older_than_the_accepted_one_is_refused() {
    let issued = RevB::default().issued_ms;
    assert_eq!(list(issued - 1), Ok(issued));
    assert_eq!(list(issued), Ok(issued), "the same list may be reloaded");
    assert_eq!(list(issued + 1), Err(PolicyError::Replayed));
}

#[test]
fn the_table_and_list_labels_are_pinned() {
    assert_eq!(Label::Policy.bytes(), b"nsh/1 policy\0");
    assert_eq!(Label::Revoke.bytes(), b"nsh/1 revoke\0");
}
