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

//! #DF and #MC: installed on their own stacks, and both end the machine
//! rather than return to it. They still clear AC first, so the report they
//! print is not made with user memory open.

use x86_64::structures::idt::InterruptStackFrame;

use crate::arch::interrupt::clear_ac;
use crate::interrupts::handlers;

pub extern "x86-interrupt" fn isr_double_fault(frame: InterruptStackFrame, code: u64) -> ! {
    clear_ac();
    handlers::double_fault(frame, code)
}

pub extern "x86-interrupt" fn isr_machine_check(frame: InterruptStackFrame) -> ! {
    clear_ac();
    handlers::machine_check(frame)
}
