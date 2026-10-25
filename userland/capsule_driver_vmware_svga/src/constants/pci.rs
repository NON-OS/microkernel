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

pub const VMWARE_VENDOR_ID: u16 = 0x15AD;
/// SVGA II: VMware's adapter, VirtualBox's VMSVGA and QEMU's vmware-svga.
pub const SVGA2_DEVICE_ID: u16 = 0x0405;
/// VirtualBox's VBoxSVGA: the same SVGA II registers under VirtualBox's own
/// id (DevVGA.cpp with VMSVGA on and the VMware id off). VBoxVGA, which has
/// no SVGA registers, carries the same id with no port window, and is left
/// alone by discovery.
pub const VBOX_VENDOR_ID: u16 = 0x80EE;
pub const VBOXSVGA_DEVICE_ID: u16 = 0xBEEF;

/// Which BAR holds the index and value ports, the framebuffer and the FIFO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub port: u8,
    pub fb: u32,
    pub fifo: u32,
}

/// VMware's order, which VirtualBox keeps under the VMware id.
pub const VMWARE_LAYOUT: Layout = Layout { port: 0, fb: 1, fifo: 2 };
/// VirtualBox's order under its own id: VRAM first, then the ports.
pub const VBOX_LAYOUT: Layout = Layout { port: 1, fb: 0, fifo: 2 };

/// The layout a function's ids call for, or none for another device.
pub const fn layout_of(vendor: u16, device: u16) -> Option<Layout> {
    match (vendor, device) {
        (VMWARE_VENDOR_ID, SVGA2_DEVICE_ID) => Some(VMWARE_LAYOUT),
        (VBOX_VENDOR_ID, VBOXSVGA_DEVICE_ID) => Some(VBOX_LAYOUT),
        _ => None,
    }
}
