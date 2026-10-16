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

//! Revocation paths for MMIO grants. Two entry points here, and a
//! device retiring takes its own (revoke.rs):
//!
//!   * `unmap_grant` — single grant, holder-pid request
//!   * `release_all_for_pid` — every grant a pid still owns
//!
//! A single grant goes from the active tables, its holder's own syscall's.
//! What a pid still owns at teardown goes from the tables it runs in, from
//! any context (K10): a thread's, or tables handed to an heir, outlive it.

use crate::hardware::broker::claim::release_all_for_pid as drop_claims_for_pid;
use crate::hardware::broker::grant::{self, GrantError, MmioGrant};
use crate::hardware::broker::holder_space::unmap_from_holder;
use crate::memory::addr::VirtAddr;

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), GrantError> {
    let g = grant::remove(pid, grant_id)?;
    unmap_one(&g);
    grant::release_user_va(pid, g.user_va, g.length);
    Ok(())
}

/// The claims first: each claimed device retires with its own grants,
/// revoked before its reset (K10b). What is left is unmapped as before.
pub fn release_all_for_pid(pid: u32, unmap_pages: bool) -> usize {
    let retired = drop_claims_for_pid(pid, unmap_pages);
    let drained = grant::drain_for_pid(pid);
    for g in &drained {
        if unmap_from_holder(pid, g.user_va, g.length) {
            grant::release_user_va(pid, g.user_va, g.length);
        } else if unmap_pages {
            unmap_one(g);
        }
    }
    retired + drained.len()
}

fn unmap_one(g: &MmioGrant) {
    let _ = crate::memory::paging::unmap_user_mmio(VirtAddr::new(g.user_va), g.length as usize);
}
