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

//! The two downgrades a bug would bring, each its own break. A stray left by
//! a keep whose cleanup the keyring refused must not win over the record just
//! written, nor come back after a forget; and two identities must never
//! resolve to each other's record.

use super::kit::{ident, level, record};
use crate::faulty::Faulty;
use nonos_bt_bond::{KeysError, Store};

/// Drive the double-fault keep that leaves the old record a stray beside the
/// new one, then return the store for the two checks below.
fn with_stray() -> Store<Faulty> {
    let mut s = Store::open(Faulty::default()).unwrap();
    s.keep(record(0, false)).unwrap(); // kept unauthenticated
    s.keys().delete = Some(KeysError::Denied); // refuse both deletes of the keep
    let _ = s.keep(record(0, true)); // re-pair up; old record left a stray
    s.keys().delete = None;
    s
}

#[test]
fn reopen_keeps_the_last_written_not_a_stale_record() {
    let s = with_stray();
    let s = Store::open(s.into_keys()).unwrap();
    assert_eq!(level(&s, 0), Some(true), "reopen must keep the record last written");
}

#[test]
fn a_stray_never_resurrects_a_forgotten_identity() {
    let mut s = with_stray();
    assert!(s.forget(0, &ident(0)).unwrap());
    assert_eq!(level(&s, 0), None, "forgotten at once");
    let s = Store::open(s.into_keys()).unwrap();
    assert_eq!(level(&s, 0), None, "a forgotten identity must not come back on reopen");
}

#[test]
fn two_identities_are_never_confused() {
    let mut s = Store::open(Faulty::default()).unwrap();
    s.keep(record(0, false)).unwrap();
    s.keep(record(1, true)).unwrap();
    assert_eq!((level(&s, 0), level(&s, 1)), (Some(false), Some(true)));
    s.forget(0, &ident(0)).unwrap();
    assert_eq!((level(&s, 0), level(&s, 1)), (None, Some(true)), "forgetting one keeps the other");
}
