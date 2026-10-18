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

use core::arch::asm;

use crate::arch::aarch64::psci::PsciMethod;

/// One fast call, `func` with up to three arguments, returning X0.
///
/// SMCCC 1.0 returns results in X0-X3 and leaves X4-X17 unpredictable;
/// 1.1 preserves X4-X17. Which one the firmware speaks is not known until it
/// has been asked, so every call is declared as 1.0: anything the compiler
/// keeps in X1-X17 across it would otherwise be lost to a 1.0 firmware.
/// X18 and up are preserved by both (DEN0028, register use).
pub fn call(conduit: PsciMethod, func: u32, args: [u64; 3]) -> i64 {
    let ret: i64;
    macro_rules! conduit_call {
        ($insn:literal) => {
            // SAFETY: the conduit is the one the device tree's PSCI node
            // named, so the instruction reaches firmware rather than an
            // undefined exception. Firmware calls touch no kernel memory
            // the compiler can see.
            unsafe {
                asm!(
                    $insn,
                    inout("x0") u64::from(func) => ret,
                    inout("x1") args[0] => _, inout("x2") args[1] => _,
                    inout("x3") args[2] => _,
                    out("x4") _, out("x5") _, out("x6") _, out("x7") _,
                    out("x8") _, out("x9") _, out("x10") _, out("x11") _,
                    out("x12") _, out("x13") _, out("x14") _, out("x15") _,
                    out("x16") _, out("x17") _,
                    options(nostack),
                )
            }
        };
    }
    match conduit {
        PsciMethod::Smc => conduit_call!("smc #0"),
        PsciMethod::Hvc => conduit_call!("hvc #0"),
    }
    ret
}
