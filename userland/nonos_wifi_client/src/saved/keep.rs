/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Forgetting saved networks, reading and pinning their flags, and sealing
//! the list before vfs keeps it.

use nonos_libc::crypto_random;
use nonos_seal::NONCE_LEN;

use super::closed::SavedFlags;
use super::error::SavedError;
use super::file::{seal_file, FILE_LEN};
use super::key::with_key;
use super::list::SavedList;
use super::pin::pin_in;
use super::store::load;
use super::write::put;

/// The flags the saved list holds for `ssid`: none for a network not saved
/// or a boot with no store, WPA3-only when the list cannot be opened.
pub fn saved_flags(ssid: &[u8]) -> SavedFlags {
    SavedFlags::from_load(&load(), ssid)
}

/// Pin the saved network `ssid` to WPA3, which only `forget` undoes. True
/// when the saved list was rewritten; a network not saved is left unsaved.
pub fn pin_wpa3(ssid: &[u8]) -> Result<bool, SavedError> {
    let mut list = load()?;
    if !pin_in(&mut list, ssid) {
        return Ok(false);
    }
    write(&list).map(|()| true)
}

/// Forget `ssid`. Forgetting the last one writes the withdrawn record, which
/// vfs accepts in any mode.
pub fn forget(ssid: &[u8]) -> Result<(), SavedError> {
    let mut list = load()?;
    let Some(i) = list.find(ssid) else {
        return Ok(());
    };
    list.remove(i);
    write(&list)
}

pub(super) fn write(list: &SavedList) -> Result<(), SavedError> {
    let mut file = [0u8; FILE_LEN];
    if !list.is_empty() {
        let mut nonce = [0u8; NONCE_LEN];
        if crypto_random(nonce.as_mut_ptr(), nonce.len()) < 0 {
            return Err(SavedError::KeyFailed);
        }
        if !with_key(|key| seal_file(list, key, &nonce, &mut file))? {
            return Err(SavedError::KeyFailed);
        }
    }
    put(&file, !list.is_empty())
}
