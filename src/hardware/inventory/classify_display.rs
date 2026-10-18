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

use super::family::HardwareFamily;
use crate::hardware::broker::{BarKind, DeviceRecord, BAR_FLAG_FIXED};

pub(super) fn classify_display(vendor: u16) -> HardwareFamily {
    match vendor {
        0x1af4 => HardwareFamily::DisplayVirtioGpu,
        0x1234 => HardwareFamily::DisplayBga,
        0x15ad => HardwareFamily::DisplayVmwareSvga,
        // VBoxSVGA, or VBoxVGA under the same id: `svga_ports` tells them apart.
        0x80ee => HardwareFamily::DisplayVmwareSvga,
        0x8086 => HardwareFamily::DisplayNativeIntel,
        0x1002 => HardwareFamily::DisplayNativeAmd,
        0x10de => HardwareFamily::DisplayNativeNvidia,
        _ => HardwareFamily::Unknown,
    }
}

/// Whether an SVGA II function has its index and value ports where its ids
/// put them: BAR0 under VMware's id, BAR1 under VirtualBox's. VBoxVGA shares
/// VBoxSVGA's id and has no such window; the broker puts its DISPI ports in
/// the first free slot, BAR1, flagged fixed, and those are not the window.
pub(super) fn svga_ports(rec: &DeviceRecord) -> bool {
    let bar = if rec.vendor == 0x80ee { 1 } else { 0 };
    let b = rec.bars[bar];
    rec.bar_count as usize > bar && b.kind == BarKind::Pio as u8 && b.flags & BAR_FLAG_FIXED == 0
}
