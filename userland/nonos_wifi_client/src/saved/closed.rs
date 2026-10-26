/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Failing closed. A saved list that cannot be opened may hold a network
//! pinned to WPA3, so every join is then held to WPA3, and a join that this
//! refuses says why with its own code.

use super::error::SavedError;
use super::list::SavedList;
use crate::join_wire::JoinFlags;

/// A join held to WPA3 because the saved list could not be opened, to a
/// network that offers only WPA2. The drivers' codes stop at -15.
pub const SAVED_UNREADABLE: i32 = -16;
/// The drivers' code for a join refused as a downgrade from WPA3.
const DOWNGRADE: i32 = -7;

/// The flags a join carries from the saved list, and whether they are there
/// because the list could not be opened.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct SavedFlags {
    pub flags: JoinFlags,
    pub closed: bool,
}

impl SavedFlags {
    /// The flags for `ssid` from a load: those saved for it, none for a
    /// network not saved or a boot with no store, and WPA3-only for every
    /// other load error.
    pub fn from_load(loaded: &Result<SavedList, SavedError>, ssid: &[u8]) -> Self {
        match loaded {
            Ok(list) => {
                let flags = list.find(ssid).map(|i| list.join_flags(i)).unwrap_or_default();
                SavedFlags { flags, closed: false }
            }
            Err(e) if e.fails_closed() => SavedFlags {
                flags: JoinFlags { wpa3_only: true, ..JoinFlags::default() },
                closed: true,
            },
            Err(_) => SavedFlags::default(),
        }
    }

    /// The join's code as the person reads it: a downgrade refused only
    /// because the list could not be opened is `SAVED_UNREADABLE`.
    pub fn code(self, code: i32) -> i32 {
        if self.closed && code == DOWNGRADE {
            SAVED_UNREADABLE
        } else {
            code
        }
    }
}
