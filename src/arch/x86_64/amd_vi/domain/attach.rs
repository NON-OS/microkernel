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

//! Moving a device's requester id between denied, identity and a capsule
//! domain by rewriting its device table entry, then dropping the cached copy
//! on every unit before returning.

use super::super::devtab::{read_dte, write_dte};
use super::super::dte::{blocked, mode_of, passthrough, translated, Dte};
use super::super::error::AmdViError;
use super::super::flush::flush_device;
use super::super::pte::LEVELS;
use super::lifetime::{root_of, ROOTS};

pub fn device_id(bus: u8, device: u8, function: u8) -> u16 {
    ((bus as u16) << 8) | (((device & 0x1F) as u16) << 3) | (function & 0x7) as u16
}

/// Put the device in `domain`. Refused while it is in another capsule's
/// domain: the broker detaches first, and a silent move would pull a device
/// out from under the capsule driving it.
pub fn attach(domain: u16, id: u16) -> Result<(), AmdViError> {
    let roots = ROOTS.lock();
    let root = root_of(&roots, domain)?;
    if mode_of(read_dte(id)?) != 0 {
        return Err(AmdViError::DeviceAlreadyAttached);
    }
    replace(id, translated(root, LEVELS, domain))
}

/// Deny the device everything, from identity or from a capsule domain.
pub fn detach(id: u16) -> Result<(), AmdViError> {
    let _roots = ROOTS.lock();
    if read_dte(id)? == blocked() {
        return Err(AmdViError::DeviceNotAttached);
    }
    replace(id, blocked())
}

/// Identity for a device the kernel enumerated, at bring-up.
pub(in crate::arch::x86_64::amd_vi) fn pass(id: u16) -> Result<(), AmdViError> {
    write_dte(id, passthrough())
}

fn replace(id: u16, entry: Dte) -> Result<(), AmdViError> {
    write_dte(id, entry)?;
    flush_device(id)
}
