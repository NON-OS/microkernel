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

//! Which of the machine's I/O APICs DMAR scopes, said once when remapping
//! comes on. Remapping an I/O APIC's lines (L44, K3) needs the requester id
//! its messages carry: with compatibility format blocked, the lines of one
//! with no scope are refused (unnamed.rs names each).

use crate::arch::x86_64::acpi::parser::ioapics;
use crate::arch::x86_64::acpi::parser::other::{ioapic_named, source_scopes_dropped};
use crate::sys::serial::Line;

pub(super) fn say_sources() {
    let all = ioapics();
    let scoped = all.iter().filter(|a| ioapic_named(a.id)).count();
    let mut line = Line::new();
    line.str(b"[VT-D] I/O APICs DMAR scopes for remapping: ").dec(scoped as u64);
    line.str(b" of ").dec(all.len() as u64);
    let dropped = source_scopes_dropped();
    if dropped > 0 {
        line.str(b", scopes not kept=").dec(dropped as u64);
    }
    line.end();
}
