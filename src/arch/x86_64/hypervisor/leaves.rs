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

/* The CPUID words detection reads, gathered so classification stays pure. */

/// First leaf of the range Intel and AMD reserve for hypervisors.
pub const LEAF_BASE: u32 = 0x4000_0000;
/// A hypervisor that presents the Hyper-V interface first (KVM with its
/// enlightenments on, Xen with viridian) moves its own signature here.
pub(super) const LEAF_ALT: u32 = 0x4000_0100;
/// "Hv#1", the interface signature in EAX of 0x40000001.
pub(super) const HV_INTERFACE: u32 = 0x3123_7648;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Leaves {
    /// CPUID.1:ECX bit 31, which every hypervisor here sets and no CPU does.
    pub present: bool,
    /// EAX of 0x40000000: the highest hypervisor leaf.
    pub max: u32,
    /// EBX, ECX, EDX of 0x40000000.
    pub sig: [u32; 3],
    /// EAX of 0x40000001, read only when `max` reaches it.
    pub interface: u32,
    /// EBX, ECX, EDX of 0x40000100, zero when the hypervisor has no such leaf.
    pub alt: [u32; 3],
}

impl Leaves {
    /// The twelve signature bytes in the order the vendor wrote them.
    pub const fn bytes(sig: [u32; 3]) -> [u8; 12] {
        let mut out = [0u8; 12];
        let mut i = 0;
        while i < 12 {
            out[i] = (sig[i / 4] >> ((i % 4) * 8)) as u8;
            i += 1;
        }
        out
    }
}
