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
 * What a Hyper-V interface lets this partition do, from CPUID 0x40000003:
 * EAX is the low half of the partition's privilege mask, EBX the high. The
 * SynIC is used only when every MSR and hypercall it needs is granted.
 */

pub(super) const LEAF_FEATURES: u32 = 0x4000_0003;

const ACCESS_SYNIC_REGS: u32 = 1 << 2;
const ACCESS_HYPERCALL_MSRS: u32 = 1 << 5;
const ACCESS_VP_INDEX: u32 = 1 << 6;
const POST_MESSAGES: u32 = 1 << 4;
const SIGNAL_EVENTS: u32 = 1 << 5;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Features {
    pub eax: u32,
    pub ebx: u32,
}

impl Features {
    /// The SynIC's MSRs, the hypercall MSRs, the VP index, and the two
    /// hypercalls VMBus rides on.
    pub(super) const fn synic_usable(self) -> bool {
        let msrs = ACCESS_SYNIC_REGS | ACCESS_HYPERCALL_MSRS | ACCESS_VP_INDEX;
        let calls = POST_MESSAGES | SIGNAL_EVENTS;
        self.eax & msrs == msrs && self.ebx & calls == calls
    }
}
