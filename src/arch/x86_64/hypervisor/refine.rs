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
 * What CPUID cannot settle, the PCI device table can. VirtualBox offers the
 * KVM or Hyper-V interface to most guests and then signs as that vendor, and
 * QEMU's TCG leaves the hypervisor bit clear on several CPU models; both
 * still put their own devices on the bus, which no physical machine carries.
 */

use super::guest::Guest;
use super::vendor::Hypervisor;

const VBOX_VENDOR: u16 = 0x80EE;
const VBOX_VMMDEV: u16 = 0xCAFE;
const VBOX_GRAPHICS: u16 = 0xBEEF;
const VIRTIO_VENDOR: u16 = 0x1AF4;
const REDHAT_VENDOR: u16 = 0x1B36;
const BOCHS_VENDOR: u16 = 0x1234;
const BOCHS_VGA: u16 = 0x1111;

pub fn refine(guest: Guest, ids: impl Iterator<Item = (u16, u16)> + Clone) -> Guest {
    let mut ids = ids;
    let vbox =
        ids.clone().any(|(v, d)| v == VBOX_VENDOR && (d == VBOX_VMMDEV || d == VBOX_GRAPHICS));
    if vbox {
        return Guest { vendor: Hypervisor::VirtualBox, ..guest };
    }
    // Only a guest that claimed nothing is promoted. A KVM guest with virtio
    // devices is still KVM, and the TCG signature already names itself.
    if guest.vendor == Hypervisor::None {
        let qemu = ids.any(|(v, d)| {
            v == VIRTIO_VENDOR || v == REDHAT_VENDOR || (v == BOCHS_VENDOR && d == BOCHS_VGA)
        });
        if qemu {
            return Guest { vendor: Hypervisor::QemuTcg, ..guest };
        }
    }
    guest
}
