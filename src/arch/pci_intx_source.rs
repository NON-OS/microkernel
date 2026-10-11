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

use crate::drivers::pci::types::PciDevice;

/// The interrupt source a PCI function's device record carries.
///
/// x86_64 and riscv64 keep config space's interrupt line, which firmware
/// assigned. aarch64 resolves the pin through the device tree's
/// `interrupt-map` and records zero for a pin the tree does not wire, so
/// the broker refuses any bind for it rather than guessing a line.
pub(crate) fn pci_intx_source(dev: &PciDevice) -> u32 {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::pci_intx::route(
        dev.bus,
        dev.device,
        dev.function,
        dev.interrupt_pin,
    )
    .map_or(0, |r| r.0);
    #[cfg(not(target_arch = "aarch64"))]
    return u32::from(dev.interrupt_line);
}
