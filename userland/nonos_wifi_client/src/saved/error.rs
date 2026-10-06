/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Why a saved network could not be read or kept.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SavedError {
    /// No NONOS store loaded on this boot, so there is nowhere to keep it.
    NoStore,
    /// The mode chosen at setup keeps nothing across boots.
    NotKept,
    /// No TPM, so no key to seal a passphrase under; none is written plain.
    NoTpm,
    /// The TPM refused: the boot state (PCRs 0, 4, 7, 9) is not the one the
    /// key belongs to, as after a firmware or kernel change.
    BootChanged,
    /// The TPM transport failed, or the key syscall was refused.
    KeyFailed,
    /// The record did not open under this machine's key: written on another
    /// machine or boot state, or altered on disk.
    Unreadable,
    /// The record is not the length or format this client writes.
    Damaged,
    /// The vfs service refused a read or write; the text is its reason.
    Vfs(&'static str),
}

impl SavedError {
    pub fn text(self) -> &'static str {
        match self {
            SavedError::NoStore => "No NONOS store on this boot's disk",
            SavedError::NotKept => "This boot keeps nothing across reboots",
            SavedError::NoTpm => "No TPM to seal the passphrase with",
            SavedError::BootChanged => "Sealed under a different boot state",
            SavedError::KeyFailed => "The TPM did not give the sealing key",
            SavedError::Unreadable => "Sealed elsewhere or altered on disk",
            SavedError::Damaged => "The saved-networks record is damaged",
            SavedError::Vfs(why) => why,
        }
    }
}
