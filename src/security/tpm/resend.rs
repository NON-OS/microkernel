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

//! `TPM_RC_RETRY`, answered as TPM 2.0 Part 2 asks: the TPM could not start
//! the command, so it did not run, and it is sent again.
//!
//! The attestation key has no `noDA`, so authorizing it is DA-protected. On a
//! TPM built from the reference code, the first such authorization after each
//! startup only records in NV that DA protection is in use and answers
//! `TPM_RC_RETRY`. So the first quote, ActivateCredential or Sign of a boot
//! meets it, and the second attempt runs. Every command goes through here.

use super::error::TpmError;
use super::transact;

const TPM_RC_RETRY: u32 = 0x0000_0922;

/// More than the one retry the DA bookkeeping costs, and still bounded.
const TRIES: usize = 3;

/// `transact`, sending `cmd` again while the TPM answers `TPM_RC_RETRY`.
///
/// # Safety
/// As `transact`: the caller owns what the command means.
pub unsafe fn transact_resending(cmd: &[u8], out: &mut [u8]) -> Result<usize, TpmError> {
    let mut tries = 1;
    loop {
        /*
         * SAFETY: eK@nonos.systems - the caller accepted the command's
         * meaning, and a command answered TPM_RC_RETRY never started, so
         * sending it again runs it at most once.
         */
        let len = unsafe { transact(cmd, out) }?;
        if !out.get(..len).is_some_and(is_retry) || tries >= TRIES {
            return Ok(len);
        }
        tries += 1;
    }
}

fn is_retry(resp: &[u8]) -> bool {
    resp.get(6..10) == Some(&TPM_RC_RETRY.to_be_bytes()[..])
}
