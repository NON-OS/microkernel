// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Failing closed: a saved list that cannot be opened holds every join to
//! WPA3, save on a boot with no store, where nothing was ever saved; and a
//! downgrade refused for that reason says so with its own code.

use super::closed::{SavedFlags, SAVED_UNREADABLE};
use super::error::SavedError;
use super::list::SavedList;
use crate::join_wire::JoinFlags;

const WPA3: JoinFlags = JoinFlags { wpa3_only: true, hidden: false };
const HIDDEN: JoinFlags = JoinFlags { wpa3_only: false, hidden: true };
const CLOSED: SavedFlags = SavedFlags { flags: WPA3, closed: true };
const OPEN_ERRORS: [SavedError; 8] = [
    SavedError::NotKept,
    SavedError::NoTpm,
    SavedError::BootChanged,
    SavedError::KeyFailed,
    SavedError::Unreadable,
    SavedError::Damaged,
    SavedError::NotSavable,
    SavedError::Vfs("refused"),
];

#[test]
fn an_opened_list_gives_the_flags_saved_and_none_for_a_stranger() {
    let mut list = SavedList::new();
    assert!(list.put_with(b"Home", b"a-good-passphrase", HIDDEN));
    let loaded = Ok(list);
    assert_eq!(
        SavedFlags::from_load(&loaded, b"Home"),
        SavedFlags { flags: HIDDEN, closed: false }
    );
    assert_eq!(SavedFlags::from_load(&loaded, b"Cafe"), SavedFlags::default());
}

#[test]
fn no_store_is_the_one_error_that_does_not_fail_closed() {
    assert!(!SavedError::NoStore.fails_closed());
    assert_eq!(SavedFlags::from_load(&Err(SavedError::NoStore), b"Home"), SavedFlags::default());
    for e in OPEN_ERRORS {
        assert!(e.fails_closed(), "{}", e.text());
        assert_eq!(SavedFlags::from_load(&Err(e), b"Home"), CLOSED, "{}", e.text());
    }
}

#[test]
fn only_a_downgrade_refused_while_closed_takes_the_new_code() {
    assert_eq!(CLOSED.code(-7), SAVED_UNREADABLE);
    for code in [0, -1, -2, -6, -8, -10, -14, -15, -101] {
        assert_eq!(CLOSED.code(code), code, "{code} passes through");
    }
    let saved_wpa3 = SavedFlags { flags: WPA3, closed: false };
    assert_eq!(saved_wpa3.code(-7), -7, "a network saved as WPA3 keeps its own reason");
    assert_eq!(SavedFlags::default().code(-7), -7);
}

#[test]
fn the_new_code_is_not_one_a_driver_sends() {
    assert!(!(-15..=-1).contains(&SAVED_UNREADABLE));
    assert_ne!(SAVED_UNREADABLE, -38);
    assert_ne!(SAVED_UNREADABLE, -101);
}
