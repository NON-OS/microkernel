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

//! `MkFamilyFact`: what the kernel knows of a Linux personality's guest
//! family, told to the anonymity fronts that decide its network grant
//! (userland/capsule_linux/design/network-grant.md). Facts only: the kernel
//! parses no policy, no box and no Linux format here; it answers from what it
//! already holds.

mod argv;
mod front;

use crate::capabilities::Capability;
use crate::process::foreign::seal;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM, ERRNO_SRCH};
use crate::usercopy::read_user_bytes;

/// Is this measurement among those the kernel recorded for the family?
pub const FACT_HOLDS: u64 = 1;
/// The generation of the live family the pid supervises, 0 for none.
pub const FACT_GENERATION: u64 = 2;
/// The pid's argv as the kernel set it at spawn.
pub const FACT_ARGV: u64 = 3;

/// `MkFamilyFact(pid, what, ptr, len)`. For FACT_HOLDS, `ptr` holds a
/// 32-byte measurement, `len` the kinds of verification asked about
/// (seal/kind.rs: LOCAL, PUBLISHER, or both), and the answer is 1 when the
/// family recorded it under one of them, else 0; a front asking for a policy
/// asks LOCAL, so nothing a publisher's chain accepted passes for one. For FACT_GENERATION, the
/// live family's generation (seal/generation.rs) or 0; for
/// FACT_ARGV, the argv is copied to `ptr` up to `len` and its whole length is
/// returned. Only an anonymity front may ask, and only of a ForeignExec
/// holder, so no other capsule's argv or measurements are told.
pub fn sys_family_fact(pid: u64, what: u64, ptr: u64, len: u64) -> i64 {
    if !front::caller_is_front() {
        return ERRNO_PERM;
    }
    let Ok(pid) = u32::try_from(pid) else { return ERRNO_INVAL };
    if !crate::process::caps::has(pid, Capability::ForeignExec.bit()) {
        return ERRNO_SRCH;
    }
    match what {
        FACT_HOLDS => {
            let Some(kinds) = asked_kinds(len) else { return ERRNO_INVAL };
            match read_user_bytes(ptr, 32).ok().and_then(|b| <[u8; 32]>::try_from(b).ok()) {
                Some(m) => i64::from(seal::holds(pid, &m, kinds)),
                None => ERRNO_FAULT,
            }
        }
        FACT_GENERATION => i64::try_from(seal::generation(pid)).unwrap_or(i64::MAX),
        FACT_ARGV => argv::copy_out(pid, ptr, len),
        _ => ERRNO_INVAL,
    }
}

/// The kinds a FACT_HOLDS question names: at least one, and none the kernel
/// does not keep.
fn asked_kinds(len: u64) -> Option<u8> {
    u8::try_from(len).ok().filter(|k| *k != 0 && *k & !seal::kind::ALL == 0)
}
