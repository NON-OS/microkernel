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

//! On x86 a device that is not behind interrupt remapping can write a
//! compatibility format message, any vector or NMI, INIT or SMI to any
//! CPU, whether or not it is ever bound: its claim is refused.

use super::refuse::Refused;
use super::say::say;

pub(super) fn admit() -> Result<(), Refused> {
    use super::open::interrupts_confined;
    use crate::memory::iommu::dev_unconfined::PRODUCTION;
    use crate::memory::iommu::dev_unremapped::OPT_IN;
    if !cfg!(target_arch = "x86_64") {
        return Ok(());
    }
    let remapping = crate::memory::iommu::capabilities().interrupt_remapping;
    if remapping {
        return Ok(());
    }
    let posture = crate::memory::iommu::no_iommu::holds();
    if interrupts_confined(remapping, OPT_IN, PRODUCTION, false) {
        say(b"interrupts not remapped, under nonos-dev-unremapped-interrupts");
        return Ok(());
    }
    if interrupts_confined(remapping, false, PRODUCTION, posture) {
        say(b"interrupts not remapped, under the no-IOMMU posture");
        return Ok(());
    }
    say(b"refused: interrupts not remapped");
    Err(Refused)
}
