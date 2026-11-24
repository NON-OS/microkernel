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

//! What the kernel knows of a Linux personality's guest family, asked by an
//! anonymity front deciding its network grant (MkFamilyFact). Only the
//! processes registered as net.anon and net.socks5 are answered.

use crate::syscall::{call_raw, N_MK_FAMILY_FACT};

const HOLDS: u64 = 1;
const GENERATION: u64 = 2;
const ARGV: u64 = 3;

/// Accepted under a local trailer this machine minted (MkLocalVerify).
pub const FAMILY_KIND_LOCAL: u8 = 1;
/// Accepted under a publisher's chain (MkCapsuleVerify).
pub const FAMILY_KIND_PUBLISHER: u8 = 2;

/// Whether the live family `pid` supervises recorded `measurement` under one
/// of `kinds` (FAMILY_KIND_*): 1 or 0, or a negative errno.
pub fn mk_family_holds(pid: u32, measurement: &[u8; 32], kinds: u8) -> i64 {
    let args = [pid as u64, HOLDS, measurement.as_ptr() as u64, u64::from(kinds), 0, 0];
    call_raw(N_MK_FAMILY_FACT, args)
}

/// The generation of the live family `pid` supervises, 0 for none, or a
/// negative errno.
pub fn mk_family_generation(pid: u32) -> i64 {
    call_raw(N_MK_FAMILY_FACT, [pid as u64, GENERATION, 0, 0, 0, 0])
}

/// `pid`'s argv as the kernel set it, NUL-separated, into `out`: its whole
/// length, or a negative errno.
pub fn mk_family_argv(pid: u32, out: &mut [u8]) -> i64 {
    call_raw(N_MK_FAMILY_FACT, [pid as u64, ARGV, out.as_mut_ptr() as u64, out.len() as u64, 0, 0])
}
