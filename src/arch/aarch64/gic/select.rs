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

//! Which GIC to drive. Pure, so the decision table is proved on the host.

use super::version::GicVersion;

/// The device tree's answer when it gave one, else the CPU's: a CPU with the
/// GICv3 system-register interface (`ID_AA64PFR0_EL1.GIC` non-zero) is wired
/// to a GICv3, and one without can only reach a GICv2 through MMIO. A tree
/// that names a GICv3 for a CPU without the registers is refused, since the
/// first `ICC_*` access would be an undefined instruction.
///
/// The opposite mismatch is allowed: a core can implement the registers and
/// still sit beside a GIC-400, which then has to be driven as one.
pub fn select(described: Option<GicVersion>, sysregs: bool) -> Result<GicVersion, &'static str> {
    match (described, sysregs) {
        (Some(GicVersion::V3), false) => {
            Err("the device tree names a GICv3 but this CPU has no GIC system registers")
        }
        (Some(version), _) => Ok(version),
        (None, true) => Ok(GicVersion::V3),
        (None, false) => Ok(GicVersion::V2),
    }
}
