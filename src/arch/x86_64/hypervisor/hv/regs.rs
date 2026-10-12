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

/* The Hyper-V MSRs the SynIC needs, and the values written to them. */

pub(super) const MSR_GUEST_OS_ID: u32 = 0x4000_0000;
pub(super) const MSR_HYPERCALL: u32 = 0x4000_0001;
pub(super) const MSR_VP_INDEX: u32 = 0x4000_0002;
pub(super) const MSR_SCONTROL: u32 = 0x4000_0080;
pub(super) const MSR_SIEFP: u32 = 0x4000_0082;
pub(super) const MSR_SIMP: u32 = 0x4000_0083;
pub(super) const MSR_EOM: u32 = 0x4000_0084;
pub(super) const MSR_SINT0: u32 = 0x4000_0090;

/// The SINT VMBus delivers on, messages and channel events alike.
pub(super) const SINT_VMBUS: u32 = 2;

/*
 * The vector the SINT raises, from the range with no gate yet (0xC1..0xF9,
 * past the broker's pool). The SINT stays masked until the gate exists:
 * an unmasked SINT with no gate would fault the guest on its first message.
 */
pub const VECTOR_HV_SYNIC: u8 = 0xC1;

const ENABLE: u64 = 1;
const SINT_MASKED: u64 = 1 << 16;
const PAGE_MASK: u64 = !0xFFF;

/*
 * The guest OS identity, in the open-source form: bit 63 set and nothing
 * else, so the host learns that the guest is an open-source system and not
 * which one or which version.
 */
pub(super) const GUEST_OS_ID: u64 = 1 << 63;

/// A page MSR (hypercall, SIMP, SIEFP): the page's address and enable.
pub(super) const fn page(gpa: u64) -> u64 {
    gpa & PAGE_MASK | ENABLE
}

pub(super) const fn sint(vector: u8, masked: bool) -> u64 {
    vector as u64 | if masked { SINT_MASKED } else { 0 }
}

pub(super) const fn scontrol_enabled() -> u64 {
    ENABLE
}
