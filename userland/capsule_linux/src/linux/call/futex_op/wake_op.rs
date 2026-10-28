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

//! FUTEX_WAKE_OP's operation, read as Linux's futex_atomic_op_inuser reads
//! it: what is done to the second word, and the test of its old value that
//! decides whether that word's waiters are woken too. Qt's QSemaphore wakes
//! through it. Pure, so the host proofs hold it.

use crate::linux::abi::errno;

const FUTEX_WAKE_OP: u64 = 5;
const FUTEX_PRIVATE_FLAG: u64 = 128;
/// The flag that makes the argument a shift, 1 << oparg.
const OPARG_SHIFT: u32 = 8;

/// The operation, its argument, the comparison and its argument.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WakeOp(u32, i32, u32, i32);

/// Whether `op` is FUTEX_WAKE_OP, private or not. With the realtime clock it
/// is not, and `decode` refuses it as Linux does.
pub fn named(op: u64) -> bool {
    op & !FUTEX_PRIVATE_FLAG == FUTEX_WAKE_OP
}

/// The operation in `val3`, or ENOSYS for an operation or a comparison Linux
/// does not know, refused here before either word is touched.
pub fn decode_op(val3: u64) -> Result<WakeOp, i64> {
    let e = val3 as u32;
    let (mut op, cmp) = (e >> 28, (e >> 24) & 0xf);
    let mut oparg = ((e << 8) as i32) >> 20;
    let cmparg = ((e << 20) as i32) >> 20;
    if op & OPARG_SHIFT != 0 {
        op &= !OPARG_SHIFT;
        oparg = 1i32.wrapping_shl(oparg as u32 & 31);
    }
    match op <= 4 && cmp <= 5 {
        true => Ok(WakeOp(op, oparg, cmp, cmparg)),
        false => Err(errno::ENOSYS),
    }
}

impl WakeOp {
    /// The second word after the operation: set, add, or, and-not or xor.
    pub fn apply(&self, old: u32) -> u32 {
        let arg = self.1 as u32;
        match self.0 {
            0 => arg,
            1 => old.wrapping_add(arg),
            2 => old | arg,
            3 => old & !arg,
            _ => old ^ arg,
        }
    }

    /// Whether the old value, an int, passes: ==, !=, <, <=, >, >=.
    pub fn holds(&self, old: u32) -> bool {
        let (old, arg) = (old as i32, self.3);
        [old == arg, old != arg, old < arg, old <= arg, old > arg, old >= arg][self.2 as usize]
    }
}
