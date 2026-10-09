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
//! is what lets a program's handler run and return to where it was. The
//! offsets are Linux's `struct rt_sigframe`, which a handler that reads its
//! ucontext (Go's does, to preempt) depends on byte for byte.

use crate::sigframe::{Entry, WORDS};
use crate::sigframe_build::build;
use crate::sigframe_read::returned;

pub(super) const RSP: usize = 15;
const RAX: usize = 13;

pub(super) fn regs() -> [u64; WORDS] {
    let mut r = [0u64; WORDS];
    for (i, w) in r.iter_mut().enumerate() {
        *w = 0x1111_0000 + i as u64; /* a distinct value per register */
    }
    r[RSP] = 0x7fff_ffe0_0000; /* a plausible stack pointer, page aligned */
    r
}

pub(super) const INFO: [u8; 128] = [0x5a; 128];

pub(super) fn entry(handler: u64, restorer: u64, signum: u32, blocked: u64) -> Entry<'static> {
    Entry { handler, restorer, signum, blocked, alt_top: None, stack: [0, 2, 0], info: &INFO }
}

pub(super) fn at(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
}

#[test]
fn a_returning_frame_restores_the_registers_the_handler_was_entered_over() {
    let saved = regs();
    let (frame, buf, enter) =
        build(&saved, &entry(0xdead_beef, 0xca11, 11, 0x1234)).expect("frame");
    /* The handler is entered at the frame, below the old stack, 16-byte down 8. */
    assert!(frame < saved[RSP] - 128);
    assert_eq!(frame % 16, 8);
    assert_eq!(enter[RSP], frame);
    assert_eq!(enter[16], 0xdead_beef); /* rip = handler */
    assert_eq!(enter[8], 11); /* rdi = signum */
    assert_eq!(enter[9], frame + 312); /* rsi = &siginfo */
    assert_eq!(enter[12], frame + 8); /* rdx = &ucontext */
    /* rt_sigreturn reads the ucontext the guest's rsp points at: frame + 8. */
    let uc = &buf[8..];
    assert_eq!(returned(uc).unwrap(), saved);
}

#[test]
fn the_syscall_return_value_rides_in_the_saved_rax() {
    let mut saved = regs();
    saved[RAX] = 42; /* the value the interrupted syscall returns */
    let (_, buf, _) = build(&saved, &entry(1, 2, 3, 0)).expect("frame");
    assert_eq!(returned(&buf[8..]).unwrap()[RAX], 42);
}
