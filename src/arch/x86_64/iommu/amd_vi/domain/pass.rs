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

//! Identity entries bring-up writes, before any capsule claims a device.

use super::super::devtab::update_dte;
use super::super::dte::passthrough;
use super::super::error::AmdViError;
use super::super::intr_format::keep_interrupts;
use super::identity::note;

/// Identity for a device the kernel enumerated, at bring-up.
pub(in crate::arch::x86_64::iommu::amd_vi) fn pass(id: u16) -> Result<(), AmdViError> {
    let (before, after) = update_dte(id, |current| keep_interrupts(passthrough(), current))?;
    note(before, after);
    Ok(())
}
