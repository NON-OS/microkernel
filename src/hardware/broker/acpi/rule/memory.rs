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

//! Which OperationRegion windows the ACPI capsule may be granted.
//!
//! Pure: it decides on facts the caller supplies, so `kernel_proofs` runs the
//! same rule the kernel enforces. The capsule runs firmware's AML, which may
//! be broken or hostile; whatever it asks for, it never gets RAM the
//! allocator owns, a window another capsule holds, the interrupt and IOMMU
//! hardware the kernel itself programs, or the legacy ports the kernel's
//! own timing, interrupts and PCI configuration path depend on.

use super::facts::Facts;
use super::refusal::Refusal;

/// A SystemMemory region: only firmware's own memory (reserved, ACPI NVS,
/// ACPI reclaimable) or address space the memory map does not describe at
/// all, which is device MMIO, and then only what no capsule has claimed.
pub fn memory(base: u64, len: u64, f: &impl Facts) -> Result<(), Refusal> {
    let end = span(base, len)?;
    if let Some(p) = f.protected(base, end) {
        return Err(Refusal::Protected(p));
    }
    if f.mmio_claimed(base, end) {
        return Err(Refusal::Claimed);
    }
    let mut bad = None;
    f.mem_kinds(base, end, &mut |k| {
        if !k.grantable() {
            bad.get_or_insert(k);
        }
    });
    match bad {
        Some(k) => Err(Refusal::Memory(k)),
        None => Ok(()),
    }
}

/// The exclusive end of a non-empty range that does not wrap.
pub(super) fn span(base: u64, len: u64) -> Result<u64, Refusal> {
    if len == 0 {
        return Err(Refusal::Empty);
    }
    base.checked_add(len).ok_or(Refusal::Wraps)
}
