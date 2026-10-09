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

use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::{init_window, read32, read8, TPM_MMIO_BASE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Interface {
    Fifo,
    Crb,
}

/// TPM_INTERFACE_ID_0, at the same offset in both register files.
const INTERFACE_ID: u32 = 0x30;
const TYPE_FIFO: u32 = 0x0;
const TYPE_CRB: u32 = 0x1;
/// A TIS 1.3 part has no identity register and reads back all ones.
const TYPE_TIS13: u32 = 0xF;
/// TPM_ACCESS_0 and its tpmRegValidSts bit, which a present FIFO part sets.
const ACCESS: u32 = 0x00;
const ACCESS_VALID: u8 = 0x80;

/// Map the window and read the interface identity. Returns the raw
/// register for the log line, and the interface when one answers.
pub(super) fn identify() -> Result<(u32, Option<Interface>), TpmError> {
    if init_window().is_err() {
        crate::log::warn!("[TPM] could not map the window at {:#X}", TPM_MMIO_BASE);
        return Err(TpmError::NotPresent);
    }
    let id = read32(INTERFACE_ID)?;
    let found = match id & 0xF {
        TYPE_FIFO => Some(Interface::Fifo),
        TYPE_CRB => Some(Interface::Crb),
        TYPE_TIS13 if fifo_answers()? => Some(Interface::Fifo),
        _ => None,
    };
    Ok((id, found))
}

/// Whether a FIFO part is decoding the access register: an empty bus reads
/// all ones, a present part reads with tpmRegValidSts set.
fn fifo_answers() -> Result<bool, TpmError> {
    let access = read8(ACCESS)?;
    Ok(access != u8::MAX && access & ACCESS_VALID != 0)
}
