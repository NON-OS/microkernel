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

use super::super::boot::info::BootInfo;
use super::geometry::{BLOCK_2M, ENTRIES, GIB};
use super::l2_pool::l2_under;
use super::{state, PageAttributes};

/// Map what the kernel reaches before any driver exists: the console and the
/// interrupt controller. The redistributors are a separate window, often
/// megabytes away, and without them no core's timer tick ever arrives.
pub(super) unsafe fn map_devices(boot_info: &BootInfo) {
    let attrs = PageAttributes::device();
    // Zero is "absent", the GICv2 redistributor or the GICv3 CPU interface:
    // mapped anyway, it turns RAM at 0, the Pi 4's first 4 MiB, into Device.
    map_device_range(boot_info.uart.base, 1, &attrs);
    map_device_range(boot_info.gic_dist_base, 1, &attrs);
    map_device_range(boot_info.gic_cpu_base, 1, &attrs);
    // A redistributor frame per CPU, 128 KiB apiece, runs past one block.
    let redist = boot_info.gic_redist_base & !(BLOCK_2M - 1);
    map_device_range(redist, 2 * BLOCK_2M, &attrs);
    // The MSI controller is the interrupt controller's third part: an ITS
    // is two 64 KiB frames, which can straddle a block boundary.
    if let Some((msi, _)) = boot_info.msi {
        map_device_range(msi.base, msi.size, &attrs);
    }
    // The bridge's I/O window and BAR memory. Config space is the MMIO
    // mapper's, once the unified address space is up.
    map_device_range(boot_info.pci_io_cpu_base, boot_info.pci_io_size, &attrs);
    map_device_range(boot_info.pci_mmio_base, boot_info.pci_mmio_size, &attrs);
    map_device_range(boot_info.rtc_base, BLOCK_2M, &attrs);
}

/// Identity map every 2MB block that a window touches.
unsafe fn map_device_range(base: u64, size: u64, attrs: &PageAttributes) {
    if base == 0 || size == 0 {
        return;
    }
    let start = base & !(BLOCK_2M - 1);
    let end = base.saturating_add(size).saturating_add(BLOCK_2M - 1) & !(BLOCK_2M - 1);
    for addr in (start..end).step_by(BLOCK_2M as usize) {
        map_device_block(addr, attrs);
    }
}

/// Identity map the 2MB block holding one MMIO window, in the level 2 table of
/// its gigabyte, which RAM there shares. Past 512 GiB, or with no table left,
/// it stays unmapped and the first access faults, by name.
unsafe fn map_device_block(addr: u64, attrs: &PageAttributes) {
    let l1_idx = (addr / GIB) as usize;
    if l1_idx >= ENTRIES {
        return;
    }
    let Some(l2) = l2_under(state::l1(), l1_idx) else {
        return;
    };
    l2.set_block(((addr / BLOCK_2M) % ENTRIES as u64) as usize, addr & !(BLOCK_2M - 1), attrs);
}
