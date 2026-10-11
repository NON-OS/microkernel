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

use super::classify::classify;
use super::leaves::{Leaves, LEAF_BASE};
use super::vendor::Hypervisor;

fn words(s: &[u8; 12]) -> [u32; 3] {
    let w = |i: usize| u32::from_le_bytes([s[i], s[i + 1], s[i + 2], s[i + 3]]);
    [w(0), w(4), w(8)]
}

fn guest(sig: &[u8; 12]) -> Leaves {
    Leaves { present: true, max: LEAF_BASE + 1, sig: words(sig), ..Leaves::default() }
}

// The vendor strings as each hypervisor's own documentation spells them.
#[test]
fn every_vendor_signature_is_named() {
    let table: [(&[u8; 12], Hypervisor); 9] = [
        (b"KVMKVMKVM\0\0\0", Hypervisor::Kvm),
        (b"TCGTCGTCGTCG", Hypervisor::QemuTcg),
        (b"VMwareVMware", Hypervisor::Vmware),
        (b"Microsoft Hv", Hypervisor::HyperV),
        (b"VBoxVBoxVBox", Hypervisor::VirtualBox),
        (b"XenVMMXenVMM", Hypervisor::Xen),
        (b"bhyve bhyve ", Hypervisor::Bhyve),
        (b"ACRNACRNACRN", Hypervisor::Acrn),
        (b" lrpepyh  vr", Hypervisor::Parallels),
    ];
    for (sig, want) in table {
        assert_eq!(classify(&guest(sig)).vendor, want, "{:?}", sig);
        assert_eq!(Hypervisor::from_code(want as u8), want);
    }
}

#[test]
fn no_hypervisor_bit_is_bare_metal_and_a_missing_range_is_unknown() {
    let bare = Leaves { sig: words(b"KVMKVMKVM\0\0\0"), ..Leaves::default() };
    assert_eq!(classify(&bare).vendor, Hypervisor::None);
    let mut l = guest(b"KVMKVMKVM\0\0\0");
    l.max = 0x16;
    assert_eq!(classify(&l).vendor, Hypervisor::Unknown);
    assert_eq!(classify(&guest(b"NotAVendor!!")).vendor, Hypervisor::Unknown);
}

// KVM with its Hyper-V enlightenments signs as Microsoft at the base leaf and
// as itself at 0x40000100; a real Hyper-V leaves the second range empty.
#[test]
fn the_hyperv_interface_does_not_hide_the_real_vendor() {
    let mut l = guest(b"Microsoft Hv");
    l.interface = u32::from_le_bytes(*b"Hv#1");
    assert!(classify(&l).hyperv_abi);
    assert_eq!(classify(&l).vendor, Hypervisor::HyperV);
    l.alt = words(b"KVMKVMKVM\0\0\0");
    let g = classify(&l);
    assert_eq!((g.vendor, g.hyperv_abi), (Hypervisor::Kvm, true));
    // The interface word is only meaningful when the range reaches it.
    l.max = LEAF_BASE;
    assert!(!classify(&l).hyperv_abi);
}
