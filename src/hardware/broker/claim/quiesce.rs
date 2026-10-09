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

use crate::drivers::pci::config::ConfigSpace;

/*
 * Stop a released device from mastering the bus. A driver turns Bus Master
 * Enable on through the config-write allowlist, and nothing turned it off:
 * a device whose driver exited kept its DMA running. With an IOMMU the
 * detach that follows denies it; without one, which is most machines, this
 * write is the only thing that stops it reaching all of memory.
 */
pub(super) fn stop_bus_master(device_id: u64) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    // Read back, so the log says what the device holds, not what was asked.
    let off = cfg.disable_bus_master().is_ok() && matches!(cfg.is_bus_master_enabled(), Ok(false));
    crate::sys::serial::print(b"[BROKER] released device ");
    crate::sys::serial::print_hex(device_id);
    crate::sys::serial::println(if off { b" bus master off" } else { b" bus master STILL ON" });
}

/*
 * The same write, ahead of the grants' teardown and without the log line the
 * release itself prints. `MkDeviceRelease` frees the device's DMA frames before
 * it drops the claim, and the claim's release is where bus mastering used to
 * stop: without an IOMMU the device could still write into frames already
 * handed to someone else in between. Only the holder may ask.
 */
pub(super) fn stop_bus_master_quietly(device_id: u64) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    let _ = cfg.disable_bus_master();
    /*
     * Read it back before the caller frees a frame. A configuration write
     * through ECAM or a VMD window is posted, so it may not have reached the
     * device yet; and writes the device posted before it stopped are still in
     * the fabric. A read's completion cannot pass either of them (PCIe 5.0,
     * 2.4.1, ordering rules), so once it returns they have landed.
     */
    if !matches!(cfg.is_bus_master_enabled(), Ok(false)) {
        crate::sys::serial::print(b"[BROKER] device ");
        crate::sys::serial::print_hex(device_id);
        crate::sys::serial::println(b" bus master STILL ON before its frames are freed");
    }
}
