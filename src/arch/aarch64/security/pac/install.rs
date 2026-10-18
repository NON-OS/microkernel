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

/// Put the kernel's IA key in APIAKey_EL1 and set SCTLR_EL1.EnIA on this CPU.
///
/// A macro, and used only in functions that never return (`kernel_entry`,
/// `aarch64_ap_entry`). From here on every function that saves its link
/// register signs it on entry and authenticates it on return. A function
/// entered before this point and returning after it would authenticate an
/// address it never signed, so the switch may only be thrown where nothing
/// below it on the stack will return. APIAKeyLo_EL1 and APIAKeyHi_EL1 are
/// written by encoding, S3_0_C2_C1_0 and _1 (Arm ARM, the PAuth key
/// registers), and EnIA is SCTLR_EL1 bit 31.
macro_rules! install_kernel_key {
    ($key:expr) => {{
        let key: [u64; 2] = $key;
        // SAFETY: the caller got `key` from `pac::prepare` or
        // `pac::secondary_key`, which only return one where this CPU
        // implements FEAT_PAuth, and runs in a function that never returns.
        unsafe {
            core::arch::asm!(
                "msr s3_0_c2_c1_0, {lo}",
                "msr s3_0_c2_c1_1, {hi}",
                "mrs {t}, sctlr_el1",
                "orr {t}, {t}, #0x80000000",
                "msr sctlr_el1, {t}",
                "isb",
                lo = in(reg) key[0],
                hi = in(reg) key[1],
                t = out(reg) _,
                options(nostack),
            );
        }
    }};
}

pub(crate) use install_kernel_key;
