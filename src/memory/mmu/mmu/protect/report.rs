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

//! What the boot log says about the ring-0 restrictions. Printed from the
//! read-back flags, so an operator can tell from the log alone whether this
//! machine is enforcing them, and a part that quietly refused a bit shows up
//! as a zero here instead of as nothing at all.

use crate::memory::mmu::ProtectionFlags;
use crate::sys::serial;

pub fn report(flags: ProtectionFlags) {
    serial::print(b"[CPU-PROT] smep=");
    bit(flags.smep_enabled);
    serial::print(b" smap=");
    bit(flags.smap_enabled);
    serial::print(b" umip=");
    bit(flags.umip_enabled);
    serial::print(b" nx=");
    bit(flags.nx_enabled);
    serial::print(b" wp=");
    bit(flags.wp_enabled);
    serial::print(b" pge=");
    bit(global_pages());
    serial::println(b"");

    if !flags.is_fully_protected() {
        // UMIP is deliberately not part of the verdict: it is absent on
        // plenty of parts that are otherwise fully protected.
        serial::println(b"[CPU-PROT] WARNING kernel is not fully protected from user pages");
    }
}

/// CR4.PGE on the calling CPU. Nothing in the kernel sets or clears it: the
/// boot CPU keeps what firmware left, an AP has what its trampoline builds.
/// Kernel PTEs carry GLOBAL, so with PGE on a CR3 reload leaves kernel entries
/// cached, and a full shootdown done that way would not remove them. Logged on
/// every CPU so a boot says which it is. This module is built for x86_64 only.
pub(super) fn global_pages() -> bool {
    super::cr4::read() & (1 << 7) != 0
}

fn bit(on: bool) {
    serial::print(if on { b"1" } else { b"0" });
}
