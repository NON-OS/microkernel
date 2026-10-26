// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! What the PC's I/O ports give x86_64 and aarch64 has no counterpart for.

/// A byte after Boot Services: there is no fixed debug UART on Arm, so this
/// says nothing until the loader takes one from the device tree or SPCR.
#[inline(always)]
pub fn post_byte(_b: u8) {}

/// A POST code: no port 0x80 here, so the code is dropped and the CPU halts.
pub fn halt_with_code(_code: u8) -> ! {
    super::halt_forever()
}

/// PCI configuration space has no I/O-port window on Arm; it is reached
/// through ECAM, which the device tree or ACPI's MCFG names. Until the loader
/// reads either, there is no device here to report.
pub fn pci_config_read(_addr: u32) -> Option<u32> {
    None
}
