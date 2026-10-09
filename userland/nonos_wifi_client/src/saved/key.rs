/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The sealing key: the TPM's HMAC over this label under an object bound to
//! the boot PCRs (the kernel's CryptoMachineKey, which needs the Crypto
//! capability). Nothing stores it; it is derived, used and wiped.

use nonos_libc::{machine_key, MACHINE_KEY_NO_TPM, MACHINE_KEY_WRONG_STATE};

use super::error::SavedError;
use crate::wipe::wipe;

/// This record's own label, so its key differs from any other label's. The
/// kernel does not tie a label to a capsule: any capsule granted Crypto that
/// names this label gets the same key.
const LABEL: &[u8] = b"wifi/saved-networks";

/// Run `f` with the key and wipe the key after.
pub(super) fn with_key<R>(f: impl FnOnce(&[u8; 32]) -> R) -> Result<R, SavedError> {
    let mut key = machine_key(LABEL).map_err(|rc| match rc {
        MACHINE_KEY_NO_TPM => SavedError::NoTpm,
        MACHINE_KEY_WRONG_STATE => SavedError::BootChanged,
        _ => SavedError::KeyFailed,
    })?;
    let out = f(&key);
    wipe(&mut key);
    Ok(out)
}
