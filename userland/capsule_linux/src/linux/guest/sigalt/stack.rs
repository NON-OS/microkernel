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

//! A thread's alternate signal stack: its flags, whether the thread is on
//! it, and what sigaltstack reports of it.

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
