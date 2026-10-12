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

use crate::arch::aarch64::{gic, pci_intx, platform_devices};
use crate::hardware::broker::irq::types::IrqBindError;

/// Set the line's trigger from the device tree, never from the capsule: a
/// PCI function's from the interrupt-map, a described device's from its own
/// `interrupts`. A line neither names is not one a capsule may hold, and
/// configuring it leaves it disabled until the bind enables it.
pub(super) fn configure(intid: u32) -> Result<(), IrqBindError> {
    let edge = pci_intx::trigger(intid).or_else(|| platform_devices::trigger(intid));
    let edge = edge.ok_or(IrqBindError::NotDeviceIrq)?;
    gic::configure_spi(intid, edge).map_err(|_| IrqBindError::PlatformError)
}
