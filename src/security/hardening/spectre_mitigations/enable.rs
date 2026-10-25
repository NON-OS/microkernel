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

//! Turning on what the part supports, and reporting only what was turned on.
//!
//! A field here means "this kernel does this", never "this CPU could". The
//! two read the same in a log and are not the same thing, and the second one
//! is how a machine ends up trusted for a mitigation nobody wired up.

use super::constants::{SPEC_CTRL_IBRS, SPEC_CTRL_SSBD, SPEC_CTRL_STIBP};
use super::cpu_apply::{apply_on_this_cpu, decide};
use super::cpuid;
use super::types::MitigationStatus;

pub fn enable_mitigations() -> MitigationStatus {
    let mut status = MitigationStatus::default();

    // Decided here for every CPU; each AP applies the same bits from its
    // own bring-up, since SPEC_CTRL is per logical CPU.
    let bits = decide();
    apply_on_this_cpu();
    status.ibrs_enabled = bits & SPEC_CTRL_IBRS != 0;
    status.ibpb_enabled = super::cpu_apply::has_ibpb();
    status.stibp_enabled = bits & SPEC_CTRL_STIBP != 0;
    status.ssbd_enabled = bits & SPEC_CTRL_SSBD != 0;

    // These two are buffer-clearing operations rather than modes, and both
    // record CPU support only. The exit hook issues VERW when md_clear is
    // present, and it runs on exec_process's first jump to user mode, not on
    // the syscall return path. l1d_flush has no caller, so L1D support is
    // recorded and the flush is never issued.
    status.mds_clear_enabled = cpuid::has_md_clear();
    status.l1d_flush_enabled = cpuid::has_l1d_flush();

    // Unconditional: kernel_entry_mitigations refills the return stack buffer
    // with no feature check on every syscall entry. The exit hook does not.
    status.rsb_stuffing_enabled = true;

    // KPTI stays false because this kernel does not implement it. It was
    // reported from CR4.PCIDE, which is process-context identifiers and has
    // nothing to do with unmapping the kernel from the user page table, so a
    // machine with PCID and no KPTI read back as protected. Parts affected by
    // Meltdown are named by `detect_vulnerabilities` and are not mitigated
    // here; saying that plainly is worth more than a field that agrees.
    status
}
