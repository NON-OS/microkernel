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

/*
 * PCI requester ids must keep device and function inside their fields.
 *
 * Both kernel encoders are included by path: the IOMMU DeviceAddress and the
 * ACPI PciDevice. Neither masked the device to five bits, so device 32 on bus
 * 0 packed as device 0 on bus 1 and named another device's DMA context; bdf
 * did not mask the function either. The checks below fail against that code.
 */

#[path = "../../../../src/memory/iommu/device.rs"]
pub mod device;
#[path = "../../../../src/arch/x86_64/acpi/devices/pci/types.rs"]
pub mod types;
mod tests;
