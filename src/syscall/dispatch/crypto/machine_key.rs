// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! `CryptoMachineKey`: a 32-byte key bound to this TPM and this boot state.
//!
//! The caller names the key with a label and gets the same bytes back every
//! boot on this machine, and different bytes on any other machine or under any
//! other kernel. Nothing is stored anywhere to make that so. Labels the kernel
//! keeps for itself are refused here, and a bound label is derived for the
//! caller's own namespace alone (`machine_key::bound_label`).

use crate::capabilities::Capability;
use crate::security::attest_registry::bound_namespace_of;
use crate::security::tpm::machine_key::{bound_label, derive, is_user_label, KeyError, LABEL_MAX};
use crate::syscall::dispatch::require_capability;
use crate::syscall::SyscallResult;

use super::machine_key_errno::errno_for;
use super::primitives::copy;
use crate::security::hardening::memory_sanitization::secure_zero_slice;

pub fn handle_machine_key(label_ptr: u64, label_len: u64, out_ptr: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    let label = match copy::read_vec(label_ptr, label_len, LABEL_MAX) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if !is_user_label(&label) {
        return crate::syscall::dispatch::errno(22);
    }
    let namespace = caller_group().and_then(bound_namespace_of);
    let label = match bound_label(&label, namespace.as_ref()) {
        Ok(l) => l,
        Err(e) => return crate::syscall::dispatch::errno(e),
    };
    match derive(&label) {
        Ok(mut key) => {
            let written = copy::write(out_ptr, &key);
            secure_zero_slice(&mut key);
            written
        }
        Err(e) => {
            // The response code says which rule the TPM applied, so it is
            // logged whole; `log TPM` in the Terminal reads it back.
            match e {
                KeyError::Refused(rc) => crate::log_warn!("[TPM] machine key: {} (rc 0x{:03X})", e.as_str(), rc),
                _ => crate::log_warn!("[TPM] machine key: {}", e.as_str()),
            }
            crate::syscall::dispatch::errno(errno_for(e))
        }
    }
}

/// The pid spawn recorded: a thread answers for the process it belongs to.
fn caller_group() -> Option<u32> {
    let pid = crate::process::current_pid()?;
    crate::process::with_process(pid, |pcb| pcb.thread_group_id()).filter(|&g| g != 0).or(Some(pid))
}
