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

//! Teardown quiesces before anything is freed (C2). The guard is the
//! release: MkDeviceRelease clears Bus Master, reads it back and only then
//! drains the device's DMA frames, so it comes last, once the controller is
//! disabled, and no queue memory is unmapped by hand before it, in any drop
//! order. The driver's own Bus Master clear ahead of it is defence in depth
//! that takes effect with the kernel's K6.

use nonos_libc::{calls, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE};

use crate::fake::{up, Up};

/// Disabled, Bus Master written clear, IRQ unbound, MMIO unmapped, released.
const LET_GO: [&str; 5] = ["disabled", "pci_command", "irq_unbind", "mmio_unmap", "device_release"];

/// The calls since the last DMA map are `want`, the release is the last
/// call of all, nothing was unmapped by hand, and the Command write keeps
/// the registers decoding and INTx quiet with Bus Master clear.
fn let_go(want: &[&str]) {
    let all = calls();
    let from = all.iter().rposition(|c| c.0 == "dma_map").map_or(0, |i| i + 1);
    assert_eq!(all[from..].iter().map(|c| c.0).collect::<Vec<_>>(), want);
    assert_eq!(all.last().map(|c| c.0), Some("device_release"), "the release is last");
    assert!(all.iter().all(|c| c.0 != "dma_unmap"), "queue memory unmapped by hand");
    let command = all.iter().rfind(|c| c.0 == "pci_command").map(|c| c.1 as u16);
    assert_eq!(command, Some(MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE));
}

#[test]
fn a_device_let_go_is_disabled_then_released_last_in_either_drop_order() {
    // The queues go first (each `let _` ends them); then the device goes
    // first, as Driver declares it.
    let Up { device, admin, io, .. } = up();
    let _ = (io, admin);
    drop(device);
    let_go(&LET_GO);
    let Up { device, admin, io, .. } = up();
    drop(device);
    let _ = (admin, io);
    let_go(&LET_GO);
}

#[test]
fn a_disable_that_failed_is_not_waited_out_again_on_the_way_out() {
    let Up { fake, mut device, .. } = up();
    fake.borrow_mut().wedged = true;
    assert!(device.disable().is_err(), "RDY never clears");
    let ticks = fake.borrow().ticks;
    drop(device);
    assert_eq!(fake.borrow().ticks, ticks, "no second CAP.TO wait");
    let_go(&LET_GO);
}

#[test]
fn a_device_never_mapped_is_released_last() {
    crate::handles::release(7, 1);
    let_go(&["pci_command", "device_release"]);
}
