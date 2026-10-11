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

//! sigaltstack: the guest's stack_t read and written.

use super::super::memory::Memory;
use super::change::change;
use super::stack::report;
use crate::linux::abi::errno;

/// `stack_t` as the guest lays it out: ss_sp, a 4-byte ss_flags and its
/// padding, then ss_size.
pub const STACK_T_LEN: usize = 24;

pub fn stack_t(raw: &[u8]) -> Option<[u64; 3]> {
    let word = |at: usize| Some(u64::from_le_bytes(raw.get(at..at + 8)?.try_into().ok()?));
    let flags = u32::from_le_bytes(raw.get(8..12)?.try_into().ok()?);
    Some([word(0)?, u64::from(flags), word(16)?])
}

pub fn stack_t_bytes(s: [u64; 3]) -> [u8; STACK_T_LEN] {
    let mut b = [0u8; STACK_T_LEN];
    b[..8].copy_from_slice(&s[0].to_le_bytes());
    b[8..12].copy_from_slice(&(s[1] as u32).to_le_bytes());
    b[16..].copy_from_slice(&s[2].to_le_bytes());
    b
}

/// sigaltstack(ss, old) by a thread at `rsp` holding `alt`, in Linux's
/// order: the new stack is read first, so `ss` and `old` may be one buffer;
/// the old one is the stack as it was, written only once the change is
/// made, and not at all when it is refused. The stack the thread then
/// holds, and the call's answer.
pub fn sigaltstack(
    mem: &impl Memory,
    alt: [u64; 3],
    rsp: u64,
    ss: u64,
    old: u64,
) -> ([u64; 3], u64) {
    let new = match ss {
        0 => None,
        at => match mem.read_at(at, STACK_T_LEN).as_deref().and_then(stack_t) {
            Some(new) => Some(new),
            None => return (alt, errno::fail(errno::EFAULT)),
        },
    };
    let was = report(alt, rsp);
    let now = match new.map(|new| change(alt, rsp, new)) {
        None => alt,
        Some(Ok(now)) => now,
        Some(Err(e)) => return (alt, errno::fail(e)),
    };
    if old != 0 && mem.write_at(old, &stack_t_bytes(was)) < STACK_T_LEN as i64 {
        return (now, errno::fail(errno::EFAULT));
    }
    (now, errno::ok(0))
}
