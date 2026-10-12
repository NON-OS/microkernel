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

//! The buses behind a root port that faces the outside of the machine,
//! from the port's bridge registers (PCI-to-PCI bridge header, dword 0x18:
//! primary, secondary and subordinate bus numbers), and whether a function
//! lies behind one. Pure: `kernel_proofs` mounts it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalRange {
    pub segment: u16,
    /// The port itself.
    pub port: (u8, u8, u8),
    pub secondary: u8,
    pub subordinate: u8,
    /// The port declared Microsoft's DmaProperty as well.
    pub dma_property: bool,
}

/// The secondary and subordinate buses, from the header type (offset 0x0E)
/// and dword 0x18: None for a function that is no PCI-to-PCI bridge, or
/// whose numbers are not a range below it.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub fn bridge_buses(header_type: u8, dword_18: u32, port_bus: u8) -> Option<(u8, u8)> {
    let [primary, secondary, subordinate, _] = dword_18.to_le_bytes();
    let bridge = header_type & 0x7F == 1;
    let ordered = primary == port_bus && secondary > primary && subordinate >= secondary;
    (bridge && ordered).then_some((secondary, subordinate))
}

/// Whether the function at `seg`, `(bus, dev, func)` is the port or lies
/// behind it.
pub fn behind(r: &ExternalRange, seg: u16, bdf: (u8, u8, u8)) -> bool {
    seg == r.segment && (bdf == r.port || (r.secondary..=r.subordinate).contains(&bdf.0))
}
