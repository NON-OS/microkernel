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

use super::guest::Guest;
use super::refine::refine;
use super::vendor::Hypervisor;

fn g(vendor: Hypervisor) -> Guest {
    Guest { vendor, hyperv_abi: true }
}

#[test]
fn virtualbox_devices_override_the_interface_it_signs_as() {
    for dev in [0xCAFE, 0xBEEF] {
        let ids = [(0x8086u16, 0x1237u16), (0x80EE, dev)];
        for v in [Hypervisor::Kvm, Hypervisor::HyperV, Hypervisor::None] {
            let r = refine(g(v), ids.iter().copied());
            assert_eq!(r.vendor, Hypervisor::VirtualBox);
            // The interface stays: VMBus capsules still key on it.
            assert!(r.hyperv_abi);
        }
    }
    // Another VirtualBox function (USB, the AC'97 is Intel's id) proves nothing.
    let r = refine(g(Hypervisor::Kvm), [(0x80EEu16, 0x0021u16)].into_iter());
    assert_eq!(r.vendor, Hypervisor::Kvm);
}

#[test]
fn qemu_devices_promote_only_a_guest_that_claimed_nothing() {
    for id in [(0x1AF4u16, 0x1000u16), (0x1B36, 0x000D), (0x1234, 0x1111)] {
        assert_eq!(refine(g(Hypervisor::None), [id].into_iter()).vendor, Hypervisor::QemuTcg);
        assert_eq!(refine(g(Hypervisor::Kvm), [id].into_iter()).vendor, Hypervisor::Kvm);
    }
    // VMSVGA's 15ad:0405 sits in VirtualBox and QEMU too: it names nobody.
    let ids = [(0x15ADu16, 0x0405u16), (0x8086, 0x100E)];
    assert_eq!(refine(g(Hypervisor::None), ids.into_iter()).vendor, Hypervisor::None);
}
