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

use super::fatal::{fatal, fatal_interrupt};

/// Which slot of the group was taken, as its vector entry passes it in x1.
const KIND_IRQ: u64 = 1;
const KIND_FIQ: u64 = 2;
const KIND_SERROR: u64 = 3;

#[no_mangle]
pub extern "C" fn aarch64_exc_invalid_sp0(frame: *mut ExceptionFrame, kind: u64) -> ! {
    let frame = unsafe { &*frame };
    match kind {
        KIND_IRQ => fatal_interrupt(b"SP_EL0 vector IRQ", None, frame),
        KIND_FIQ => fatal_interrupt(b"SP_EL0 vector FIQ", None, frame),
        KIND_SERROR => fatal(b"SP_EL0 vector SError", frame),
        _ => fatal(b"SP_EL0 vector sync", frame),
    }
}

#[no_mangle]
pub extern "C" fn aarch64_exc_invalid_aarch32(frame: *mut ExceptionFrame, kind: u64) -> ! {
    let frame = unsafe { &*frame };
    match kind {
        KIND_IRQ => fatal_interrupt(b"AArch32 vector IRQ", None, frame),
        KIND_FIQ => fatal_interrupt(b"AArch32 vector FIQ", None, frame),
        KIND_SERROR => fatal(b"AArch32 vector SError", frame),
        _ => fatal(b"AArch32 vector sync", frame),
    }
}
