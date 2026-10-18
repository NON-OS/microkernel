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

//! Each operation decoded, or refused with Linux's errno.

use crate::linux::abi::errno::{EINVAL, ENOSYS};
use crate::linux::call::futex_op::{decode, Cmd};
use crate::linux::guest::futex_pick::MATCH_ANY;

const WAIT: u64 = 0;
const WAKE: u64 = 1;
const REQUEUE: u64 = 3;
const CMP_REQUEUE: u64 = 4;
const WAIT_BITSET: u64 = 9;
const WAKE_BITSET: u64 = 10;
const PRIVATE: u64 = 128;
const REALTIME: u64 = 256;
pub(super) const W: u64 = 0x7000_1000;

/// uaddr, op, uaddr2, val3, and what the decoder answers.
type Row = (u64, u64, u64, u64, Result<Cmd, i64>);

#[test]
fn each_operation_decodes_or_is_refused_with_linux_errno() {
    let table: [Row; 17] = [
        (W, WAIT, 0, 0, Ok(Cmd::Wait { bits: MATCH_ANY, absolute: false })),
        (W, WAIT | PRIVATE, 0, 7, Ok(Cmd::Wait { bits: MATCH_ANY, absolute: false })),
        (W, WAIT_BITSET | PRIVATE | REALTIME, 0, 6, Ok(Cmd::Wait { bits: 6, absolute: true })),
        (W, WAKE | PRIVATE, 0, 0, Ok(Cmd::Wake { bits: MATCH_ANY })),
        (W, WAKE_BITSET, 0, 0x8000_0000, Ok(Cmd::Wake { bits: 0x8000_0000 })),
        (W, REQUEUE, W + 4, 0, Ok(Cmd::Requeue { compare: false })),
        (W, CMP_REQUEUE | PRIVATE, W, 0, Ok(Cmd::Requeue { compare: true })),
        // A zero bitset matches nothing and is refused.
        (W, WAIT_BITSET, 0, 0, Err(EINVAL)),
        (W, WAKE_BITSET, 0, 1 << 32, Err(EINVAL)),
        // The realtime clock is WAIT_BITSET's alone.
        (W, WAIT | REALTIME, 0, 0, Err(ENOSYS)),
        (W, WAKE | REALTIME, 0, 0, Err(ENOSYS)),
        // Operations not served, and bits outside the command, are ENOSYS:
        // 0x200 is not FUTEX_WAIT with a stray bit.
        (W, 5, 0, 0, Err(ENOSYS)),
        (W, 6 | PRIVATE, 0, 0, Err(ENOSYS)),
        (W, 0x200, 0, 0, Err(ENOSYS)),
        // A word is four bytes on a four-byte boundary, both of them.
        (W + 2, WAIT, 0, 0, Err(EINVAL)),
        (W + 1, WAKE, 0, 0, Err(EINVAL)),
        (W, CMP_REQUEUE, W + 3, 0, Err(EINVAL)),
    ];
    for (uaddr, op, uaddr2, val3, want) in table {
        assert_eq!(decode(uaddr, op, uaddr2, val3), want, "{uaddr:#x} op {op:#x}");
    }
}
