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

pub const BUS_KIND_PCI: u8 = 1;
pub const BUS_KIND_ACPI: u8 = 2;
pub const BUS_KIND_VIRT: u8 = 3;
/// A device-tree device: `device` is its interface (`FDT_SDHCI` and on),
/// the PCI class fields the class the same registers carry on PCI.
pub const BUS_KIND_FDT: u8 = 4;
pub const FDT_SDHCI: u16 = 1;
pub const FDT_XHCI: u16 = 2;
pub const FDT_AHCI: u16 = 3;
/// What firmware hands over at boot: today only the boot framebuffer.
pub const BUS_KIND_FIRMWARE: u8 = 5;
