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

//! The SystemIO and PCI_Config halves of the region rule.

use super::facts::Facts;
use super::legacy::LEGACY_PORTS;
use super::memory::span;
use super::refusal::Refusal;

/// A SystemIO region: any port range no capsule holds, outside the legacy
/// devices the kernel owns.
pub fn io(base: u64, len: u64, f: &impl Facts) -> Result<(), Refusal> {
    let end = span(base, len)?;
    if end > 0x1_0000 {
        return Err(Refusal::Wraps);
    }
    if let Some((_, _, which)) = LEGACY_PORTS.iter().find(|(lo, hi, _)| base < *hi && *lo < end) {
        return Err(Refusal::Legacy(*which));
    }
    if f.ports_claimed(base as u16, (end - base) as u32) {
        return Err(Refusal::Claimed);
    }
    Ok(())
}

/// A PCI_Config access: reads of any function, writes only to one no other
/// capsule has claimed, since its driver owns its configuration.
pub fn pci_config(bdf: (u16, u8, u8, u8), write: bool, f: &impl Facts) -> Result<(), Refusal> {
    if write && f.pci_claimed(bdf) {
        return Err(Refusal::Claimed);
    }
    Ok(())
}
