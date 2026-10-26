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

//! The IPI gate functions `install` puts in the IDT. Each clears AC first:
//! an IPI can land while ring 3 runs with it set, and with AC set SMAP does
//! not fault.

use x86_64::structures::idt::InterruptStackFrame;

use super::super::handlers;
use crate::arch::interrupt::clear_ac;

pub(super) extern "x86-interrupt" fn tlb_shootdown(_frame: InterruptStackFrame) {
    clear_ac();
    handlers::tlb_shootdown();
}

pub(super) extern "x86-interrupt" fn reschedule(_frame: InterruptStackFrame) {
    clear_ac();
    handlers::reschedule();
}

pub(super) extern "x86-interrupt" fn call_function(_frame: InterruptStackFrame) {
    clear_ac();
    handlers::call_function();
}

pub(super) extern "x86-interrupt" fn barrier(_frame: InterruptStackFrame) {
    clear_ac();
    handlers::barrier();
}

// These two never return, but an interrupt gate takes a handler typed as
// returning unit, so the divergence stays inside the call rather than in the
// signature.
pub(super) extern "x86-interrupt" fn panic(_frame: InterruptStackFrame) {
    clear_ac();
    handlers::panic()
}

pub(super) extern "x86-interrupt" fn stop(_frame: InterruptStackFrame) {
    clear_ac();
    handlers::stop()
}
