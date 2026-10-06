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

use super::area::in_register_window;
use super::regs::{
    LOCALITY_MS, TPM_INTERFACE_ID, TPM_INTF_TYPE_CRB, TPM_INTF_TYPE_MASK, TPM_LOC_CTRL,
    TPM_LOC_CTRL_RELINQUISH, TPM_LOC_CTRL_REQUEST, TPM_LOC_STS, TPM_LOC_STS_GRANTED,
};
use super::wait::until;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::{init_window, read32, write32};

/// Map the window and confirm a CRB part is behind it.
///
/// The transport already chose CRB; this re-reads the identity because the
/// two register files overlap, and driving a FIFO part through CRB offsets
/// writes command bytes into control registers and produces a garbled
/// response rather than an error. A control area outside the window (AMD's
/// firmware TPM) has no window and no identity register to read.
pub(super) fn probe() -> Result<(), TpmError> {
    if !in_register_window()? {
        return Ok(());
    }
    init_window()?;
    let intf = read32(TPM_INTERFACE_ID)?;
    if intf == u32::MAX {
        return Err(TpmError::NotPresent);
    }
    if intf & TPM_INTF_TYPE_MASK != TPM_INTF_TYPE_CRB {
        return Err(TpmError::NotPresent);
    }
    Ok(())
}

/// Take locality 0. Every command runs inside a granted locality; issuing one
/// without it is answered by the part, not by this driver. A control area of
/// its own has no locality registers: that part runs every command at 0.
pub(super) fn acquire() -> Result<(), TpmError> {
    if !in_register_window()? {
        return Ok(());
    }
    if read32(TPM_LOC_STS)? & TPM_LOC_STS_GRANTED != 0 {
        return Ok(());
    }
    // SAFETY: eK@nonos.systems - a locality request changes no key state and
    // is the documented way to begin using the part.
    unsafe { write32(TPM_LOC_CTRL, TPM_LOC_CTRL_REQUEST)? };
    until(LOCALITY_MS, || Ok(read32(TPM_LOC_STS)? & TPM_LOC_STS_GRANTED != 0))
}

/// Give locality back. Best effort: a part that will not release it is not a
/// reason to fail a command that already succeeded.
pub(super) fn release() {
    if !matches!(in_register_window(), Ok(true)) {
        return;
    }
    // SAFETY: eK@nonos.systems - relinquishing only narrows what this driver
    // may do next.
    let _ = unsafe { write32(TPM_LOC_CTRL, TPM_LOC_CTRL_RELINQUISH) };
}
