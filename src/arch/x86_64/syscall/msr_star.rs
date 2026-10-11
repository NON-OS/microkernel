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

use super::msr::write_msr;
use super::msr_regs::IA32_STAR;

// Encode STAR for x86_64 SYSCALL/SYSRET.
//
// Layout per Intel SDM Vol 2B: STAR[31:0] is reserved in 64-bit mode;
// STAR[47:32] is the SYSCALL selector base — the CPU loads CS from
// `STAR[47:32]` and SS from `STAR[47:32] + 8`; STAR[63:48] is the
// SYSRET selector base — the CPU loads SS from `STAR[63:48] + 8` and
// CS from `STAR[63:48] + 16`. CS is forced to RPL=3, but SS is loaded
// verbatim, so STAR[63:48] must itself carry RPL=3; otherwise user mode
// runs with an RPL=0 SS and the next return to CPL=3 raises #GP(SS).
// The kernel CS/SS pair and the user CS/SS pair must each sit at
// 8-byte-adjacent slots in the GDT, with the user pair laid out as
// (USER_SS, USER_CS) immediately above STAR[63:48].
//
// Caller passes the **raw** GDT selectors (RPL=0). With our GDT:
// kernel_code_raw=0x08, kernel_data_raw=0x10, user_data_raw=0x18,
// user_code_raw=0x20. STAR[63:48] is (user_data_raw - 8) | 3 = 0x13
// so that:
//   SYSRET SS = 0x13 + 8  = 0x1B = USER_DATA
//   SYSRET CS = 0x13 + 16 = 0x23 = USER_CODE
pub fn setup_star(kernel_code_raw: u16, user_data_raw: u16) -> Result<(), &'static str> {
    if kernel_code_raw & 0x7 != 0 {
        return Err("kernel CS selector is not RPL0 aligned");
    }
    if user_data_raw & 0x7 != 0 {
        return Err("user data selector is not RPL0 aligned");
    }
    if user_data_raw < 8 {
        return Err("user data selector is below SYSRET base");
    }

    let sysret_base = ((user_data_raw - 8) | 3) as u64;
    let syscall_field = (kernel_code_raw as u64) << 32;
    let sysret_field = sysret_base << 48;
    write_msr(IA32_STAR, syscall_field | sysret_field);
    Ok(())
}
