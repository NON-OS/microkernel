/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Reading and keeping saved networks through vfs.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::dropped::Dropped;
use super::error::SavedError;
use super::file::{is_withdrawn, open_file, FILE_LEN, PATH};
use super::keep::write;
use super::key::with_key;
use super::list::SavedList;
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
/// network the radio is associated with through WPA3-SAE (AKM 8 or 24), or
/// whose access point set Transition Disable in message 3, is saved as WPA3,
/// so no later join, from a panel or at boot, accepts WPA2 for it.
pub fn remember(ssid: &[u8], pass: &[u8]) -> Result<Option<Dropped>, SavedError> {
    let wpa3_only = find().and_then(|d| d.link()).is_some_and(|l| l.pins_wpa3_on(ssid));
    remember_with(ssid, pass, JoinFlags { wpa3_only, hidden: false })
}

/// Remember `ssid` with `pass` and `flags`: a hidden network, which a probe
/// request may name, is saved this way. Flags already saved for the network
/// are kept. The network dropped to make room, if one was, is returned for
/// the panel to name on screen.
pub fn remember_with(
    ssid: &[u8],
    pass: &[u8],
    flags: JoinFlags,
) -> Result<Option<Dropped>, SavedError> {
    let mut list = match load() {
        Ok(list) => list,
        Err(SavedError::Unreadable | SavedError::Damaged | SavedError::BootChanged) => {
            SavedList::new()
        }
        Err(e) => return Err(e),
    };
    let Some(dropped) = list.put_dropping(ssid, pass, flags) else {
        return Err(SavedError::NotSavable);
    };
    write(&list).map(|()| dropped)
}
