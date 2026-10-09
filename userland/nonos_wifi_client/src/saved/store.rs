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
use crate::driver::find;
use crate::join_wire::JoinFlags;

/// The saved networks. An absent or withdrawn record is an empty list.
pub fn load() -> Result<SavedList, SavedError> {
    if !vfs::store_has_disk() {
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
/// (sealed under an earlier boot state) is replaced rather than kept. A
/// network the radio is associated with through WPA3-SAE is saved as WPA3,
/// so no later join, from a panel or at boot, accepts WPA2 for it.
pub fn remember(ssid: &[u8], pass: &[u8]) -> Result<(), SavedError> {
    let wpa3_only = find().and_then(|d| d.link()).is_some_and(|l| l.joined_with_sae(ssid));
    remember_with(ssid, pass, JoinFlags { wpa3_only, hidden: false })
}

/// Remember `ssid` with `pass` and `flags`: a hidden network, which a probe
/// request may name, is saved this way. Flags already saved for the network
/// are kept.
pub fn remember_with(ssid: &[u8], pass: &[u8], flags: JoinFlags) -> Result<(), SavedError> {
    let mut list = match load() {
        Ok(list) => list,
        Err(SavedError::Unreadable | SavedError::Damaged | SavedError::BootChanged) => {
            SavedList::new()
        }
        Err(e) => return Err(e),
    };
    if !list.put_with(ssid, pass, flags) {
        return Err(SavedError::Damaged);
    }
    write(&list)
}

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
