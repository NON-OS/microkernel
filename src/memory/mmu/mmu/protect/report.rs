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
    let bit = |on: bool| if on { b"1" } else { b"0" };
    let mut l = serial::Line::new();
    l.str(b"[CPU-PROT] smep=").str(bit(flags.smep_enabled));
    l.str(b" smap=").str(bit(flags.smap_enabled)).str(b" umip=").str(bit(flags.umip_enabled));
    l.str(b" nx=").str(bit(flags.nx_enabled)).str(b" wp=").str(bit(flags.wp_enabled));
    l.str(b" pge=").str(bit(flags.pge_enabled)).end();

    if !flags.is_fully_protected() {
        // UMIP is deliberately not part of the verdict: it is absent on
        // plenty of parts that are otherwise fully protected.
        serial::println(b"[CPU-PROT] WARNING kernel is not fully protected from user pages");
    }
}
