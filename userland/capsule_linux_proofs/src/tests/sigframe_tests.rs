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

//! The signal frame a handler enters through, and the frame it returns from,
//! are the same layout read two ways: build one, then read the sigcontext
//! back the way rt_sigreturn does, and the registers must be identical. This
//! is what lets a program's handler run and return to where it was.

use crate::sigframe::{build, returned, WORDS};

pub(super) const RSP: usize = 15;
const RAX: usize = 13;

/// A distinct value per register, and a plausible, page-aligned stack pointer.
pub(super) fn regs() -> [u64; WORDS] {
    let mut r = [0u64; WORDS];
    for (i, w) in r.iter_mut().enumerate() {
        *w = 0x1111_0000 + i as u64;
    }
    r[RSP] = 0x7fff_ffe0_0000;
    r
}

#[test]
fn a_returning_frame_restores_the_registers_the_handler_was_entered_over() {
    let saved = regs();
    let (frame, buf, enter) =
        build(&saved, 0xdead_beef, 0xca11, 11, 0x1234, None, false).expect("frame");
    /* The handler is entered at the frame, below the old stack, 16-byte down 8. */
    assert!(frame < saved[RSP] - 128);
    assert_eq!(enter[RSP], frame);
    /* rip is the handler, rdi the signal, rdx the ucontext. */
    assert_eq!(enter[16], 0xdead_beef);
    assert_eq!(enter[8], 11);
    assert_eq!(enter[12], frame + 8);
    /* rt_sigreturn reads the ucontext the guest's rsp points at: frame + 8. */
    let uc = &buf[8..];
    assert_eq!(returned(uc).unwrap(), saved);
}

#[test]
fn the_syscall_return_value_rides_in_the_saved_rax() {
    let mut saved = regs();
    /* 0 as the thread trapped, then the value the interrupted call returns. */
    saved[RAX] = 0;
    saved[RAX] = 42;
    let (_, buf, _) = build(&saved, 1, 2, 3, 0, None, false).expect("frame");
    assert_eq!(returned(&buf[8..]).unwrap()[RAX], 42);
}

#[test]
fn a_stack_too_low_to_hold_a_frame_is_refused() {
    let mut low = regs();
    /* Below the red zone plus a frame. */
    low[RSP] = 64;
    assert!(build(&low, 1, 2, 3, 0, None, false).is_none());
}
