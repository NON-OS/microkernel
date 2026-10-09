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

//! The console lines AMD-Vi events print, each built whole.

use super::event::{self, Event};
use crate::arch::x86_64::iommu::unit::fault::bdf_text;
use crate::sys::serial::Line;

/// "[AMD-VI] IOMMU event write dev=02:00.0 addr=... domain=... page fault".
pub(super) fn log_event(base: u64, raw: Event) {
    let code = event::code(raw);
    let mut line = Line::new();
    line.str(b"[AMD-VI] IOMMU event ");
    if code == event::IO_PAGE_FAULT {
        line.str(if event::is_write(raw) { b"write " } else { b"read " });
    }
    line.str(b"dev=").str(&bdf_text(event::device_id(raw)));
    line.str(b" addr=").hex(event::address(raw));
    line.str(b" domain=").dec(event::domain(raw) as u64);
    line.str(b" flags=").hex(event::flags(raw) as u64);
    line.str(b" unit=").hex(base).str(b" ");
    line.str(event::code_text(code)).end();
}

pub(super) fn log_hidden(hidden: u32, overflowed: bool) {
    let mut line = Line::new();
    if overflowed {
        line.str(b"[AMD-VI] IOMMU event log overflowed; some events were lost").end();
    } else {
        line.str(b"[AMD-VI] IOMMU events not shown=").dec(hidden as u64).end();
    }
}
