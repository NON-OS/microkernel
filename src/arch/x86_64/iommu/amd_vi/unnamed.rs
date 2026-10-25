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

//! An IO-APIC no IVHD special entry names has no requester id to remap its
//! lines by, so each of its lines is refused, as on VT-d. Said once
//! at boot, by id, so a keyboard that never types has a visible cause.

use crate::arch::x86_64::acpi::parser::other::amd_ioapic_requester;
use crate::sys::serial::Line;

pub(super) fn say_unnamed_ioapics() {
    for io in crate::arch::x86_64::acpi::ioapics() {
        if amd_ioapic_requester(io.id).is_none() {
            let mut line = Line::new();
            line.str(b"[AMD-VI] IO-APIC id=").dec(io.id as u64);
            line.str(b" gsi_base=").dec(io.gsi_base as u64);
            line.str(b" is in no IVHD special entry: its lines are refused").end();
        }
    }
}
