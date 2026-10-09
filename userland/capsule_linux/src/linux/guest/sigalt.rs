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

//! A thread's alternate signal stack, kept as Linux's do_sigaltstack keeps
//! it: `[ss_sp, ss_flags, ss_size]`, the flags as the guest set them, and
//! no stack at all whenever the size is zero. Pure, so the host proofs hold
//! what sigaltstack reports, what it accepts, and when a thread is on it.

use crate::linux::abi::errno;

use super::memory::Memory;

/// ss_flags: running on the alternate stack, and no alternate stack set.
pub const SS_ONSTACK: u64 = 1;
pub const SS_DISABLE: u64 = 2;
/// Disarm the stack each time a handler is entered on it, and put it back
/// when that handler returns, so a handler may switch away from it.
pub const SS_AUTODISARM: u64 = 1 << 31;
/// MINSIGSTKSZ on x86-64.
pub const MINSIGSTKSZ: u64 = 2048;

/// The stack a thread starts with: none.
pub const NONE: [u64; 3] = [0, SS_DISABLE, 0];

/// True when `rsp` is on the stack `alt`, as Linux's on_sig_stack has it:
/// above its base and no higher than its top. A stack set with
/// SS_AUTODISARM never counts, since entering a handler disarms it.
pub fn on_stack(alt: [u64; 3], rsp: u64) -> bool {
    let [sp, flags, size] = alt;
    flags & SS_AUTODISARM == 0 && size != 0 && rsp > sp && rsp - sp <= size
}

/// What sigaltstack reports to a thread at `rsp`: ss_sp, ss_flags, ss_size,
/// the flags saying disabled, on it, or neither, with SS_AUTODISARM kept.
pub fn report(alt: [u64; 3], rsp: u64) -> [u64; 3] {
    let [sp, flags, size] = alt;
    let now = match (size, on_stack(alt, rsp)) {
        (0, _) => SS_DISABLE,
        (_, true) => SS_ONSTACK,
        _ => 0,
    };
    [sp, now | (flags & SS_AUTODISARM), size]
}

/// The stack a thread at `rsp` holding `alt` has once it asks for `new`
/// (ss_sp, ss_flags, ss_size as the guest wrote them), or the errno Linux
/// refuses with: EPERM while the thread runs on its stack, EINVAL for a mode
/// other than none, SS_ONSTACK or SS_DISABLE, ENOMEM for a stack smaller
/// than MINSIGSTKSZ. Disabling forgets the base and size.
pub fn change(alt: [u64; 3], rsp: u64, new: [u64; 3]) -> Result<[u64; 3], i64> {
    if on_stack(alt, rsp) {
        return Err(errno::EPERM);
    }
    let [sp, flags, size] = new;
    match flags & !SS_AUTODISARM {
        SS_DISABLE => Ok([0, flags, 0]),
        0 | SS_ONSTACK if size < MINSIGSTKSZ => Err(errno::ENOMEM),
        0 | SS_ONSTACK => Ok([sp, flags, size]),
        _ => Err(errno::EINVAL),
    }
}

/// The stack a thread is left with once a handler is entered: none, when it
/// was set with SS_AUTODISARM; otherwise as it was.
pub fn entered(alt: [u64; 3]) -> [u64; 3] {
    match alt[1] & SS_AUTODISARM {
        0 => alt,
        _ => NONE,
    }
}

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
