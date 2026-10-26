/*
 * NONOS Operating System (AGPL-3.0-or-later)
 */

//! The panel offers "Forget all" (and `F` withdraws the whole record) for
//! exactly the load errors that hold every join to WPA3: all but no store,
//! where nothing was ever saved and there is nothing to withdraw.

use super::error::SavedError;

#[test]
fn forget_all_is_offered_for_every_load_error_but_no_store() {
    assert!(!SavedError::NoStore.fails_closed());
    for e in [
        SavedError::NoTpm,
        SavedError::BootChanged,
        SavedError::KeyFailed,
        SavedError::Unreadable,
        SavedError::Damaged,
    ] {
        assert!(e.fails_closed(), "{}", e.text());
    }
}

#[test]
fn a_changed_boot_says_what_changed() {
    let why = SavedError::BootChanged.text();
    assert!(why.contains("kernel") && why.contains("firmware") && why.contains("Secure Boot"));
}
