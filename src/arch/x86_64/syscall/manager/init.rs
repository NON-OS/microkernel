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

use core::sync::atomic::{AtomicBool, Ordering};

use super::program::program_this_cpu;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static BOOT_CPU_PROGRAMMED: AtomicBool = AtomicBool::new(false);

// Programs LSTAR/STAR/SFMASK and enables EFER.SCE so the `syscall`
// instruction at CPL=3 enters `syscall_entry_asm` at CPL=0.
//
// STAR is encoded so SYSRET delivers CS = SEL_USER_CODE (0x23) and
// SS = SEL_USER_DATA (0x1B) — see `msr::setup_star` for the SDM
// derivation. SYSCALL delivers CS = SEL_KERNEL_CODE (0x08) and
// SS = SEL_KERNEL_DATA (0x10).
pub fn init() -> Result<(), &'static str> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err("syscall already initialized");
    }
    program_this_cpu()?;
    BOOT_CPU_PROGRAMMED.store(true, Ordering::SeqCst);
    Ok(())
}

/// The same registers on an application processor. They are per CPU, and one
/// that never had them programmed took the first `syscall` from user mode as
/// an invalid opcode. Refused until the boot CPU has run `init`, so the checks
/// there have passed once before any other CPU relies on the same encoding.
pub fn init_ap() -> Result<(), &'static str> {
    if !BOOT_CPU_PROGRAMMED.load(Ordering::SeqCst) {
        return Err("syscall not initialized on the boot cpu");
    }
    program_this_cpu()
}
