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

//! The PSCI firmware interface, and which instruction reaches it.
//!
//! PSCI leaves the conduit to the platform (Arm DEN0022): SMC to EL3
//! firmware, HVC to a hypervisor. The device tree's `/psci` node says which,
//! in its `method` string. QEMU virt starting a kernel at EL1 says "hvc", and
//! an SMC there, with no EL3, is undefined.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

/// PSCI 0.2 and later use the function IDs DEN0022 fixes. A node that is
/// only "arm,psci" (0.1) names its own IDs in properties, which the kernel's
/// calls do not read, so it is not taken.
const NEEDLES: &[&[u8]] = &[b"arm,psci-1.0", b"arm,psci-0.2"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conduit {
    Smc,
    Hvc,
}

/// `None` on a board whose firmware has no PSCI, such as a Raspberry Pi 4 on
/// its stock firmware, and on a node whose method is missing, unreadable or
/// neither: no call can be made there that is known to arrive.
pub fn find(fdt: &Fdt) -> Result<Option<Conduit>, FdtError> {
    for found in fdt.devices(NEEDLES) {
        let (node, _) = found?;
        let Some(method) = fdt.property(&node, b"method")? else {
            continue;
        };
        match method.string() {
            Ok(b"smc") => return Ok(Some(Conduit::Smc)),
            Ok(b"hvc") => return Ok(Some(Conduit::Hvc)),
            _ => continue,
        }
    }
    Ok(None)
}
