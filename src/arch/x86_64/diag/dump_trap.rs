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

use x86_64::structures::idt::InterruptStackFrame;

use super::cpl::cpl_from_cs;
use crate::arch::x86_64::paging::read_cr3;
use crate::sys::serial::Line;

pub fn dump_trap(name: &[u8], frame: &InterruptStackFrame, err: Option<u64>, cr2: Option<u64>) {
    let cs = frame.code_segment as u64;
    let cpl = cpl_from_cs(cs);
    let rip = frame.instruction_pointer.as_u64();
    let rsp = frame.stack_pointer.as_u64();
    let ss = frame.stack_segment as u64;
    let rflags = frame.cpu_flags;
    let cr3 = read_cr3();
    let asid = crate::memory::paging::manager::active_asid().unwrap_or(0xFFFF_FFFF);
    let pid = crate::process::current_pid().unwrap_or(0);

    // One line, through the fatal writer: a trap can come while the port's
    // lock is held, and the line must still be written, whole.
    let mut l = Line::new();
    l.str(b"[TRAP ").str(name).str(b"] cpl=").str(&[b'0' + cpl]);
    l.str(b" rip=").hex_lower(rip).str(b" rsp=").hex_lower(rsp).str(b" cs=").hex_lower(cs);
    l.str(b" ss=").hex_lower(ss).str(b" rflags=").hex_lower(rflags);
    l.str(b" cr3=").hex_lower(cr3).str(b" asid=").hex_lower(asid as u64);
    l.str(b" pid=").hex_lower(pid as u64);
    if let Some(e) = err {
        l.str(b" err=").hex_lower(e);
    }
    if let Some(c) = cr2 {
        l.str(b" cr2=").hex_lower(c);
    }
    l.end_fatal();
    if let (3, Some(c)) = (cpl, cr2) {
        super::walk_fault::print_walk(cr3, c);
    }
}
