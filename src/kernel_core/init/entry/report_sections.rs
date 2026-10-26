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

//! Whether the kernel image is mapped the way its own section table declares.
//!
//! Reported rather than enforced. The table's symbols were defined in no
//! linker script until recently, so every reader of it was dead-code
//! eliminated and this question had never been asked of a running kernel.
//! Printing it is also what keeps the table out of the linker's dead-code
//! pass.
//!
//! A count alone was not enough to act on: the first run said 3 of 4 and
//! nothing about which, so each fault now names its page and what the
//! hardware grants there against what the section asked for.

use crate::memory::hardening::{section_first_fault, SectionFault};
use crate::memory::layout;
use crate::sys::serial;

pub(super) fn report_kernel_sections() {
    let sections = layout::kernel_sections();
    let mut faults = 0;
    for section in sections.iter() {
        if let Some(fault) = section_first_fault(section) {
            faults += 1;
            report_fault(&fault);
        }
    }
    let mut l = serial::Line::new();
    l.str(b"[KSEC] ").dec((sections.len() - faults) as u64).str(b"/").dec(sections.len() as u64);
    l.str(if faults == 0 {
        b" sections mapped as declared"
    } else {
        b" sections mapped as declared, WARNING W^X not held"
    });
    l.end();
}

fn report_fault(fault: &SectionFault) {
    let mut l = serial::Line::new();
    l.str(b"[KSEC] fault at ").hex(fault.va);
    match fault.granted {
        None => l.str(b" no mapping, wanted"),
        Some(granted) => {
            flags(l.str(b" got"), granted.writable, granted.executable).str(b" wanted")
        }
    };
    flags(&mut l, fault.want_writable, fault.want_executable).end();
}

fn flags(l: &mut serial::Line, writable: bool, executable: bool) -> &mut serial::Line {
    l.str(if writable { b" w=1" } else { b" w=0" });
    l.str(if executable { b" x=1" } else { b" x=0" })
}
