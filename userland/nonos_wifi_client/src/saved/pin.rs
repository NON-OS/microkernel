/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Pinning a saved network to WPA3 once a join shows it runs WPA3 (SAE, or
//! the access point's Transition Disable). Only the person's Forget clears a
//! pin: the saved flags only add.

use super::list::{SavedList, PASS_MAX};
use crate::join_wire::JoinFlags;
use crate::wipe::wipe;

/// Pin `ssid` in `list` to WPA3, keeping its passphrase and other flags.
/// True when the list changed; false for a network not saved or already
/// pinned. The passphrase's copy is wiped before this returns.
pub fn pin_in(list: &mut SavedList, ssid: &[u8]) -> bool {
    let Some(i) = list.find(ssid) else {
        return false;
    };
    let flags = list.join_flags(i);
    if flags.wpa3_only {
        return false;
    }
    let mut pass = [0u8; PASS_MAX];
    let n = list.passphrase(i).len().min(PASS_MAX);
    pass[..n].copy_from_slice(&list.passphrase(i)[..n]);
    let changed = list.put_with(ssid, &pass[..n], JoinFlags { wpa3_only: true, ..flags });
    wipe(&mut pass);
    changed
}
