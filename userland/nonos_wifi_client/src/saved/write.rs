/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Handing the record to vfs to keep on disk.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;
use nonos_policy_proto::Field;

use super::error::SavedError;
use super::file::{DIR, FILE_LEN, PATH};
use super::key::with_key;

/// Persist `bytes` at the record's path. A record that holds networks is
/// written only when the policy store says this boot keeps state (vfs refuses
/// it otherwise too); the withdrawn record is written in any mode. A record
/// loaded from an earlier boot belongs to nobody, and only a file's owner may
/// persist it, so it is unlinked and written afresh.
pub(super) fn put(bytes: &[u8], holds_networks: bool) -> Result<(), SavedError> {
    if holds_networks && !keeps_state() {
        return Err(SavedError::NotKept);
    }
    let pid = mk_getpid();
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, DIR);
    let _ = vfs::unlink(pid, PATH);
    vfs::write_file(pid, PATH, bytes).map_err(SavedError::Vfs)?;
    vfs::persist(pid, PATH).map_err(SavedError::Vfs)
}

/// Withdraw the whole record, as for one this machine can no longer open.
pub fn forget_all() -> Result<(), SavedError> {
    put(&[0u8; FILE_LEN], false)
}

/// Whether a network could be sealed and kept on this boot: vfs loaded a
/// NONOS store and the TPM gives the key. Whether the mode keeps state is
/// checked when the record is written, since setup sets it only at the end.
pub fn sealing_ready() -> Result<(), SavedError> {
    if vfs::store_settled() != Ok(true) || vfs::store_status() != Ok(0) {
        return Err(SavedError::NoStore);
    }
    with_key(|_| ())
}

/// Whether the mode chosen at setup keeps state across boots.
pub fn keeps_state() -> bool {
    let port = nonos_policy_client::lookup();
    port.and_then(|p| nonos_policy_client::get_bool(p, Field::Persistent)) == Some(true)
}
