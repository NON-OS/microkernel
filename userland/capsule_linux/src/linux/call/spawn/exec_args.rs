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

//! Reading a guest's argv or envp. Written over the guest's memory, so the
//! host proofs hold every refusal.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::file::cstr::cstr;
use crate::linux::guest::memory::Memory;
use crate::linux::guest::STACK_SIZE;

/// Enough for any real command line. A caller handing over more than
/// this is not going to be helped by us trying.
const MAX_ENTRIES: usize = 4096;

/// Linux's own ceiling on one argument, thirty-two pages.
const MAX_ARG: usize = 32 * 4096;

/// The whole vector, a quarter of the stack the guest wakes on.
const MAX_TOTAL: usize = STACK_SIZE as usize / 4;

/// E2BIG, which execve answers for a vector too large to pass.
const E2BIG: i64 = 7;

/// The vector at `array`, or execve's errno: EFAULT for a slot or a string
/// that cannot be read, E2BIG for one string past MAX_ARG, more than
/// MAX_ENTRIES strings, or more than MAX_TOTAL bytes in all. A vector past
/// MAX_ENTRIES was cut there and the program started with only those, an
/// argument list it was never given.
pub fn vector(mem: &impl Memory, mut array: u64) -> Result<Vec<Vec<u8>>, i64> {
    let mut out = Vec::new();
    if array == 0 {
        return Ok(out);
    }
    let mut total = 0usize;
    loop {
        let slot = mem.read_at(array, 8).ok_or(errno::EFAULT)?;
        let ptr = u64::from_le_bytes(slot.as_slice().try_into().map_err(|_| errno::EFAULT)?);
        if ptr == 0 {
            return Ok(out);
        }
        if out.len() == MAX_ENTRIES {
            return Err(E2BIG);
        }
        let arg = cstr(mem, ptr, MAX_ARG).map_err(|e| match e {
            errno::ENAMETOOLONG => E2BIG,
            e => e,
        })?;
        /*
         * Each string costs its bytes and the terminator the stack
         * block will need for it.
         */
        total = total.saturating_add(arg.len() + 1);
        if total > MAX_TOTAL {
            return Err(E2BIG);
        }
        out.push(arg);
        array = array.checked_add(8).ok_or(errno::EFAULT)?;
    }
}
