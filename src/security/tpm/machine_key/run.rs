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

//! The one place this module talks to the TPM.
//!
//! Every command in the derivation goes through here, which is what makes the
//! safety argument checkable: it is one `unsafe` block with one claim to
//! verify, rather than six call sites each making it separately.

use alloc::vec::Vec;

use super::error::KeyError;
use crate::security::tpm::crb::transact;

/// Longer than any response this module can provoke. The largest is the
/// create, whose public area and private blob together stay well inside this;
/// `transact` refuses to overrun the buffer rather than truncating into it.
const RESPONSE_MAX: usize = 1024;

pub(super) fn run(cmd: &[u8]) -> Result<Vec<u8>, KeyError> {
    let mut buf = [0u8; RESPONSE_MAX];
    /*
     * SAFETY: eK@nonos.systems - every command built by this module creates
     * or flushes a transient object, starts a session, or reads one. None
     * writes NV storage and none names a persistent handle, so a failure
     * here cannot leave the TPM in a state that outlives the boot.
     */
    let len = unsafe { transact(cmd, &mut buf) }?;
    Ok(buf[..len].to_vec())
}
