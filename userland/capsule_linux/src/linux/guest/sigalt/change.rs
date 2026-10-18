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

//! A change to a thread's alternate signal stack, as Linux's do_sigaltstack
//! takes it, and the stack once a handler is entered on it.

use crate::linux::abi::errno;

use super::stack::{on_stack, MINSIGSTKSZ, NONE, SS_AUTODISARM, SS_DISABLE, SS_ONSTACK};

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
