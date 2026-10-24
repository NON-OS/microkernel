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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::iommus::iommus;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::device::{PlatformDevice, MAX_WINDOWS};
use super::irq::first_spi;
use super::kind::PlatformKind;

/// Everything the broker publishes about one node. A device with no
/// register window is nothing a capsule can drive.
pub(super) fn describe(
    fdt: &Fdt,
    node: &NodeRef,
    kind: PlatformKind,
) -> Result<PlatformDevice, FdtError> {
    let mut device = PlatformDevice { kind, ..PlatformDevice::EMPTY };
    for index in 0..MAX_WINDOWS {
        let Some(window) = fdt.mmio(node, index)? else {
            break;
        };
        device.windows[index] = window;
        device.window_count = index + 1;
    }
    if device.window_count == 0 {
        return Err(FdtError::BadCells);
    }
    device.irq = first_spi(fdt, node)?;
    device.dma = fdt.dma_window(node)?;
    device.coherent = fdt.property(node, b"dma-coherent")?.is_some();
    device.iommus = iommus(fdt, node)?;
    Ok(device)
}
