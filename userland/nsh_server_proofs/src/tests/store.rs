//! `RoleStore`: the table under the pinned key and no older than the host
//! last accepted, the list under that table and never older than the one
//! it replaces; a failed install keeps the list held; and a store that can
//! no longer vouch fails every session closed.

use crate::kit::roles_build::keys::{key, K};
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::table::{TableB, SERIAL};
use crate::kit::roles_build::{Spoil, BASE_ID, HOUR, NOW};
use nsh_roles::PolicyError;
use nsh_server::{RoleStore, Roles};

pub(super) fn store() -> RoleStore {
    let t = TableB::default();
    RoleStore::new(t.bytes(), t.anchor, 0, RevB::default().bytes()).expect("the store")
}

#[test]
fn the_table_is_held_to_its_pin_and_its_serial() {
    let t = TableB::default();
    let rev = RevB::default().bytes();
    let other = key(K::AuthB).id();
    assert_eq!(RoleStore::new(t.bytes(), other, 0, rev.clone()).err(), Some(PolicyError::Signer));
    assert_eq!(
        RoleStore::new(t.bytes(), t.anchor, SERIAL + 1, rev.clone()).err(),
        Some(PolicyError::Replayed)
    );
    let spoiled = TableB { spoil: Spoil::Ml, ..TableB::default() };
    assert_eq!(
        RoleStore::new(spoiled.bytes(), t.anchor, 0, rev.clone()).err(),
        Some(PolicyError::Signature)
    );
    let s = RoleStore::new(t.bytes(), t.anchor, SERIAL, rev).expect("the same serial reloads");
    assert_eq!(s.serial(), SERIAL);
}

#[test]
fn a_list_is_never_replaced_by_an_older_one() {
    let s = store();
    let newer = RevB { issued_ms: NOW - HOUR / 2, ..RevB::default() };
    s.install_list(newer.bytes()).expect("a newer list");
    assert_eq!(s.issued_ms(), NOW - HOUR / 2);
    s.install_list(newer.bytes()).expect("the same list again");
    assert_eq!(s.install_list(RevB::default().bytes()), Err(PolicyError::Replayed));
    assert_eq!(s.issued_ms(), NOW - HOUR / 2, "the list held stays");
}

#[test]
fn a_list_the_table_does_not_trust_is_refused_and_the_held_one_stays() {
    let s = store();
    let fleet = RevB { fleet: [0xF3; 16], ids: vec![BASE_ID], ..RevB::default() };
    assert_eq!(s.install_list(fleet.bytes()), Err(PolicyError::Fleet));
    let spoiled = RevB { spoil: Spoil::Ed, ids: vec![BASE_ID], ..RevB::default() };
    assert_eq!(s.install_list(spoiled.bytes()), Err(PolicyError::Signature));
    let stranger =
        RevB { signer_id: key(K::Stranger).id(), signer: K::Stranger, ..RevB::default() };
    assert_eq!(s.install_list(stranger.bytes()), Err(PolicyError::Signer));
    assert_eq!(s.install_list(vec![0; 10]).map_err(|_| ()), Err(()));
    let held = s.with(|_, list| list.contains(&BASE_ID)).expect("the held list");
    assert!(!held);
}
