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

//! The ring-0 restrictions on a CPU other than the boot one.
//!
//! CR4, CR0 and EFER are per CPU. The boot CPU sets them once, through
//! `MMU::initialize`, and an application processor that was never told ran
//! user code with SMEP, SMAP and UMIP off. Each one applies them for itself,
//! says what its own read-back was, and is held to the boot CPU's answer: a
//! CPU that ended up weaker than the one the machine was vetted on must not
//! run user code.

use crate::memory::mmu::error::MmuResult;
use crate::memory::mmu::ProtectionFlags;

/// Apply the restrictions on the calling CPU and read them back.
pub fn apply_this_cpu() -> MmuResult<ProtectionFlags> {
    super::apply()
}

/// Whether `this` has every restriction the boot CPU has.
pub fn matches_boot(this: &ProtectionFlags) -> bool {
    let Ok(boot) = crate::memory::mmu::protection_flags() else {
        return false;
    };
    (this.smep_enabled || !boot.smep_enabled)
        && (this.smap_enabled || !boot.smap_enabled)
        && (this.umip_enabled || !boot.umip_enabled)
        && (this.nx_enabled || !boot.nx_enabled)
        && (this.wp_enabled || !boot.wp_enabled)
}

/// One line for `cpu`, built whole so it does not interleave with another
/// CPU printing its own.
pub fn report_cpu(cpu: u32, flags: &MmuResult<ProtectionFlags>) {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[CPU-PROT] cpu=").dec(cpu as u64);
    match flags {
        Ok(f) => {
            l.str(b" smep=").dec(f.smep_enabled as u64);
            l.str(b" smap=").dec(f.smap_enabled as u64);
            l.str(b" umip=").dec(f.umip_enabled as u64);
            l.str(b" nx=").dec(f.nx_enabled as u64);
            l.str(b" wp=").dec(f.wp_enabled as u64);
        }
        Err(_) => {
            l.str(b" FAIL no-execute absent");
        }
    }
    l.end();
}
