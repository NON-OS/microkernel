/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Forgetting saved networks, reading their flags, and sealing the list
//! before vfs keeps it.

use nonos_libc::crypto_random;
use nonos_seal::NONCE_LEN;

use super::error::SavedError;
use super::file::{seal_file, FILE_LEN};
use super::key::with_key;
use super::list::SavedList;
use super::store::load;
use super::write::put;
use crate::join_wire::JoinFlags;

/// The flags the saved list holds for `ssid`: none for a network not saved,
/// or when the list cannot be opened on this boot (then the passphrase is
/// not at hand either, and the person typing it decides).
pub fn saved_flags(ssid: &[u8]) -> JoinFlags {
    let Ok(list) = load() else {
        return JoinFlags::default();
    };
    list.find(ssid).map(|i| list.join_flags(i)).unwrap_or_default()
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
