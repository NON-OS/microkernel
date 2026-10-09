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

/*
 * `MkDeviceSecret(out_ptr, out_len)`: this machine's device secret, the
 * witness of the anonymous device proof, as four field words little-endian in
 * the 32 bytes at `out_ptr`. The TPM derives it afresh on every call, under a
 * policy only an approved chain on this machine satisfies, and the kernel
 * keeps no copy. Needs DeviceSecret, which nonos.prove alone holds.
 */

use crate::syscall::microkernel::errnos::{ERRNO_ACCES, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_NOENT, ERRNO_PERM};
use crate::security::hardening::memory_sanitization::{secure_zero, secure_zero_slice};
use crate::security::tpm::device_secret::{device_secret, from_boot};
use crate::security::tpm::machine_key::KeyError;

/// Four words of eight bytes; a buffer of any other length is refused.
const SECRET_LEN: usize = 32;

/// 0 with the secret written; ENOENT while it stays sealed, EACCES when the TPM
/// refuses the policy, ENODEV when the TPM cannot derive it.
pub(in crate::syscall::microkernel) fn sys_device_secret(out_ptr: u64, out_len: u64) -> i64 {
    if !super::gate::device_secret_caller() {
        return ERRNO_PERM;
    }
    if out_len != SECRET_LEN as u64 {
        return ERRNO_INVAL;
    }
    /* A buffer the caller cannot take is refused before the TPM is asked. */
    if crate::usercopy::validate_user_write(out_ptr, SECRET_LEN).is_err() {
        return ERRNO_FAULT;
    }
    /* No release key, or no approval under it: the secret stays sealed. */
    let Some(approval) = from_boot() else {
        return ERRNO_NOENT;
    };
    let mut words = match device_secret(&approval) {
        Ok(words) => words,
        /* The TPM refused the policy: this is not an approved chain. */
        Err(KeyError::Refused(_)) => return ERRNO_ACCES,
        Err(_) => return ERRNO_NODEV,
    };
    let mut bytes = [0u8; SECRET_LEN];
    for (out, word) in bytes.chunks_exact_mut(8).zip(words.iter()) {
        out.copy_from_slice(&word.to_le_bytes());
    }
    let written = crate::usercopy::write_user_bytes(out_ptr, &bytes);
    secure_zero_slice(&mut bytes);
    secure_zero(words.as_mut_ptr().cast::<u8>(), core::mem::size_of_val(&words));
    match written {
        Ok(()) => 0,
        Err(_) => ERRNO_FAULT,
    }
}
