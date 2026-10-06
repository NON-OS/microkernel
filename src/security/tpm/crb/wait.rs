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

use core::sync::atomic::{fence, Ordering};

use super::area::read;
use super::regs::{
    COMPLETE_MS, READY_MS, TPM_CRB_CTRL_START, TPM_CRB_CTRL_STS, TPM_CRB_START_GO,
    TPM_CRB_STS_TPM_IDLE,
};
use crate::security::tpm::error::TpmError;

fn now_ms() -> u64 {
    crate::time::now_ns() / 1_000_000
}

/// Wait, by the clock, until `done` says so or `ms` have passed.
pub(super) fn until(ms: u64, mut done: impl FnMut() -> Result<bool, TpmError>) -> Result<(), TpmError> {
    let deadline = now_ms().saturating_add(ms);
    loop {
        if done()? {
            return Ok(());
        }
        if now_ms() >= deadline {
            /* One last look: the deadline may have passed while the part finished. */
            return if done()? { Ok(()) } else { Err(TpmError::Timeout) };
        }
        core::hint::spin_loop();
    }
}

/// The part has left idle and is ready for a command.
pub(super) fn wait_ready() -> Result<(), TpmError> {
    until(READY_MS, || Ok(read(TPM_CRB_CTRL_STS)? & TPM_CRB_STS_TPM_IDLE == 0))
}

/// The part clears the start bit when the response is ready. Waiting on that,
/// rather than on a status flag, is what the CRB interface defines.
pub(super) fn wait_complete() -> Result<(), TpmError> {
    until(COMPLETE_MS, || Ok(read(TPM_CRB_CTRL_START)? & TPM_CRB_START_GO == 0))?;
    fence(Ordering::SeqCst);
    Ok(())
}
