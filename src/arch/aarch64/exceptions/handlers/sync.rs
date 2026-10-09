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

use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::arch::aarch64::fpu::{enable as enable_fp, try_enable_for_current_task};
use crate::arch::trap::contract::deliver;
use crate::process::signal::SIGILL;
use crate::sys::serial::Line;

use super::svc;

#[no_mangle]
pub extern "C" fn aarch64_exc_sync_current(frame: *mut ExceptionFrame) -> ! {
    let frame = unsafe { &*frame };
    deliver(frame)
}

const EC_FP_ACCESS: u8 = 0x07;
const EC_SVC64: u8 = 0x15;

#[no_mangle]
pub extern "C" fn aarch64_exc_sync_lower(frame: *mut ExceptionFrame) {
    let frame = unsafe { &mut *frame };
    let ec = ((frame.esr >> 26) & 0x3F) as u8;
    if ec == EC_SVC64 {
        svc::dispatch(frame);
        return;
    }
    if ec == EC_FP_ACCESS {
        if try_enable_for_current_task() {
            return;
        }
        refuse_fp(frame)
    }
    deliver(frame)
}

/// An EL0 task touched the vector registers and there is nowhere to keep its
/// FP/SIMD state. Enabling the unit for it would hand it the registers of
/// whichever task last used them, so the task is ended with SIGILL and the
/// CPU goes on to the next one.
///
/// The trap means CPACR_EL1.FPEN was 0b00, which traps EL1 too, and the exit
/// path is compiled code that uses the vector registers. So the unit is
/// enabled for the kernel first. The task never returns to EL0, and the next
/// task's `prepare_incoming` sets FPEN again before it runs.
fn refuse_fp(frame: &ExceptionFrame) -> ! {
    enable_fp();
    Line::new()
        .str(b"[FPU] refused: EL0 FP/SIMD access with no per-task FP slot, pid=")
        .dec(u64::from(crate::process::current_pid().unwrap_or(0)))
        .str(b" elr=")
        .hex(frame.elr)
        .str(b" signal=SIGILL")
        .end();
    crate::process::terminate_current_with_signal(SIGILL)
}
