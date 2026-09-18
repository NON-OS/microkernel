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
    serial::print(b"[KSEC] ");
    serial::print_dec((sections.len() - faults) as u64);
    serial::print(b"/");
    serial::print_dec(sections.len() as u64);
    if faults == 0 {
        serial::println(b" sections mapped as declared");
    } else {
        serial::println(b" sections mapped as declared, WARNING W^X not held");
    }
}

fn report_fault(fault: &SectionFault) {
    serial::print(b"[KSEC] fault at ");
    serial::print_hex(fault.va);
    match fault.granted {
        None => serial::print(b" no mapping, wanted"),
        Some(granted) => {
            serial::print(b" got");
            flags(granted.writable, granted.executable);
            serial::print(b" wanted");
        }
    }
    flags(fault.want_writable, fault.want_executable);
    serial::println(b"");
}

fn flags(writable: bool, executable: bool) {
    serial::print(if writable { b" w=1" } else { b" w=0" });
    serial::print(if executable { b" x=1" } else { b" x=0" });
}
