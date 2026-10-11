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

//! The frame byte for byte against Linux's `struct rt_sigframe`, and where a
//! frame goes: below the alternate stack's top, or nowhere when the stack is
//! too low to hold one. A handler that reads its ucontext (Go's does, to
//! preempt) finds each field where Linux puts it.

use super::sigframe_tests::{at, entry, regs, INFO, RSP};
use crate::sigframe::SIGCONTEXT_OFF;
use crate::sigframe_build::build;

#[test]
fn the_frame_is_linux_rt_sigframe_byte_for_byte() {
    /*
     * pretcode at 0; ucontext at 8 with uc_mcontext at +40, oldmask its 22nd
     * word, uc_sigmask at +296; siginfo at 312.
     */
    assert_eq!(SIGCONTEXT_OFF, 40);
    let saved = regs();
    let (_, buf, _) = build(&saved, &entry(1, 0xca11, 3, 0xabcd)).expect("frame");
    assert_eq!(at(&buf, 0), 0xca11);
    assert_eq!(at(&buf, 8 + 40), saved[0]); /* r8 */
    assert_eq!(at(&buf, 8 + 40 + 16 * 8), saved[16]); /* rip */
    assert_eq!(at(&buf, 8 + 40 + 21 * 8), 0xabcd); /* oldmask */
    assert_eq!(at(&buf, 8 + 296), 0xabcd); /* uc_sigmask */
    assert_eq!(&buf[312..440], &INFO[..]);
    assert_eq!(buf[8 + 24], 2); /* uc_stack.ss_flags: SS_DISABLE */
}

#[test]
fn a_frame_for_the_alternate_stack_sits_below_its_top() {
    let saved = regs();
    let mut e = entry(1, 2, 3, 0);
    e.alt_top = Some(0x5000_0000);
    let (frame, _, enter) = build(&saved, &e).expect("frame");
    assert!(frame < 0x5000_0000 && frame > 0x5000_0000 - 1024);
    assert_eq!(enter[RSP], frame);
}

#[test]
fn a_stack_too_low_to_hold_a_frame_is_refused() {
    let mut low = regs();
    low[RSP] = 64; /* below the red zone plus a frame */
    assert!(build(&low, &entry(1, 2, 3, 0)).is_none());
}
