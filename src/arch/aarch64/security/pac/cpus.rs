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

//! The other CPUs' part: the boot CPU's key, and each CPU's key slot.

use super::slot::bind;
use super::state::{active, kernel_key};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::id::mpidr_affinity;
use crate::arch::aarch64::cpu::{has_feature, CpuFeature};

/// A secondary's key, if the boot CPU turned signing on. A secondary that
/// cannot sign cannot run a thread whose frames another CPU signed.
pub fn secondary_key() -> Result<Option<[u64; 2]>, &'static str> {
    if !active() {
        return Ok(None);
    }
    if !has_feature(CpuFeature::Pauth) {
        return Err("the boot CPU signs return addresses and this CPU cannot");
    }
    Ok(Some(kernel_key()))
}

/// Point this CPU's TPIDR_EL1 at its key slot, once the roster can name it.
/// Changes no key, so any function may call it.
pub fn bind_this_cpu() -> bool {
    if !active() {
        return false;
    }
    roster::index_of(mpidr_affinity()).is_some_and(|cpu| bind(cpu, kernel_key()))
}
