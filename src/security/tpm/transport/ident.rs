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

//! What the part itself says it is.

use super::ident_rule::classify;
pub(super) use super::ident_rule::Interface;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::{init_window, read32, read8, tpm_mmio_base};

/// TPM_INTERFACE_ID_0, at the same offset in both register files.
const INTERFACE_ID: u32 = 0x30;
/// TPM_ACCESS_0, which a FIFO part answers (ident_rule.rs says how).
const ACCESS: u32 = 0x00;

/// Map the window and read the interface identity. Returns the raw
/// register for the log line, and the interface when one answers.
pub(super) fn identify() -> Result<(u32, Option<Interface>), TpmError> {
    let Some(base) = tpm_mmio_base() else {
        crate::log::warn!("[TPM] no TPM register window on this platform");
        return Err(TpmError::NotPresent);
    };
    if init_window().is_err() {
        crate::log::warn!("[TPM] could not map the window at {:#X}", base);
        return Err(TpmError::NotPresent);
    }
    let id = read32(INTERFACE_ID)?;
    Ok((id, classify(id, read8(ACCESS)?)))
}
