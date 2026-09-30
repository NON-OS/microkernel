/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Reading, keeping and forgetting saved networks through vfs.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{crypto_random, mk_getpid};
use nonos_seal::NONCE_LEN;

use super::error::SavedError;
use super::file::{is_withdrawn, open_file, seal_file, FILE_LEN, PATH};
use super::key::with_key;
use super::list::SavedList;
use super::write::put;

/// The saved networks. An absent or withdrawn record is an empty list.
pub fn load() -> Result<SavedList, SavedError> {
    if vfs::store_settled() != Ok(true) || vfs::store_status() != Ok(0) {
        return Err(SavedError::NoStore);
    }
    let Ok(raw) = vfs::read_file(mk_getpid(), PATH, FILE_LEN as u32) else {
        return Ok(SavedList::new());
    };
    if is_withdrawn(&raw) {
        return Ok(SavedList::new());
    }
    with_key(|key| open_file(&raw, key))?
}

/// Remember `ssid` with `pass`. A record this machine cannot open any more
/// (sealed under an earlier boot state) is replaced rather than kept.
pub fn remember(ssid: &[u8], pass: &[u8]) -> Result<(), SavedError> {
    let mut list = match load() {
        Ok(list) => list,
        Err(SavedError::Unreadable | SavedError::Damaged | SavedError::BootChanged) => {
            SavedList::new()
        }
        Err(e) => return Err(e),
    };
    if !list.put(ssid, pass) {
        return Err(SavedError::Damaged);
    }
    write(&list)
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

fn write(list: &SavedList) -> Result<(), SavedError> {
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
