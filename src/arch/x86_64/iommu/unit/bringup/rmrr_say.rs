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

//! The reserved memory regions firmware declared, said once at bring-up. An
//! unclaimed device reaches its regions in a domain holding them alone (K2);
//! one a capsule claims has them mapped one to one in the capsule's domain
//! before it moves (L60).

use crate::arch::x86_64::acpi::parser::other::{rmrr_dropped, rmrr_regions};
use crate::sys::serial::Line;

pub(super) fn say() {
    for r in rmrr_regions().iter() {
        let mut line = Line::new();
        line.str(b"[VT-D] RMRR ").hex(r.base).str(b"-").hex(r.limit);
        line.str(b" segment=").hex(r.segment as u64);
        line.str(b" devices=").hex(r.count as u64);
        if r.truncated {
            line.str(b" truncated");
        }
        line.end();
    }
    let dropped = rmrr_dropped();
    if dropped > 0 {
        let mut line = Line::new();
        line.str(b"[VT-D] WARNING RMRR left out=").hex(dropped as u64);
        line.str(b"; malformed or past the room kept").end();
    }
}
