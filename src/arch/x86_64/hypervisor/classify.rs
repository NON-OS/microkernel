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

/* From the CPUID signature to a vendor. */

use super::guest::Guest;
use super::leaves::{Leaves, HV_INTERFACE, LEAF_BASE};
use super::vendor::Hypervisor;

const SIGNATURES: &[(&[u8; 12], Hypervisor)] = &[
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

fn lookup(sig: [u32; 3]) -> Option<Hypervisor> {
    let bytes = Leaves::bytes(sig);
    SIGNATURES.iter().find(|(s, _)| **s == bytes).map(|(_, v)| *v)
}

pub fn classify(l: &Leaves) -> Guest {
    if !l.present {
        return Guest { vendor: Hypervisor::None, hyperv_abi: false };
    }
    // A max below the base means the hypervisor answers no range at all; the
    // leaf then echoes the highest basic leaf and its words mean nothing.
    if l.max < LEAF_BASE {
        return Guest { vendor: Hypervisor::Unknown, hyperv_abi: false };
    }
    let hyperv_abi = l.max > LEAF_BASE && l.interface == HV_INTERFACE;
    let mut vendor = lookup(l.sig).unwrap_or(Hypervisor::Unknown);
    // "Microsoft Hv" at the base is only the interface on offer. The vendor
    // that really runs the guest says so at 0x40000100 when it is not Hyper-V.
    if vendor == Hypervisor::HyperV {
        if let Some(real) = lookup(l.alt) {
            vendor = real;
        }
    }
    Guest { vendor, hyperv_abi }
}
