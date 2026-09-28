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

//! A handler installed with SA_ONSTACK is entered on the thread's alternate
//! stack, and `uc_stack` in its frame says which stack that is.

use super::sigframe_tests::{regs, RSP};
use crate::sigframe::{build, returned};

/// A 32 KiB alternate stack well away from the thread's own stack.
const ALT: (u64, u64) = (0x7000_0000_0000, 0x8000);

/// ss_sp, ss_flags and ss_size, from the frame's ucontext.
fn uc_stack(buf: &[u8]) -> (u64, u64, u64) {
    let word = |at: usize| u64::from_le_bytes(buf[8 + at..8 + at + 8].try_into().unwrap());
    (word(16), word(24) & 0xffff_ffff, word(32))
}

#[test]
fn an_onstack_handler_is_entered_at_the_top_of_the_alternate_stack() {
    let saved = regs();
    let (frame, buf, enter) = build(&saved, 1, 2, 3, 0, Some(ALT), true).expect("frame");
    let (sp, size) = ALT;
    assert!(frame > sp && frame < sp + size, "frame {frame:#x} is on the alternate stack");
    assert!(sp + size - frame < 512, "and at its top");
    assert_eq!(enter[RSP], frame);
    /* The thread's own rsp is what rt_sigreturn gives back. */
    assert_eq!(returned(&buf[8..]).unwrap(), saved);
    /* uc_stack names the alternate stack; the thread was not on it. */
    assert_eq!(uc_stack(&buf), (sp, 0, size));
}

#[test]
fn a_handler_without_sa_onstack_stays_on_the_thread_stack() {
    let saved = regs();
    let (frame, buf, _) = build(&saved, 1, 2, 3, 0, Some(ALT), false).expect("frame");
    assert!(frame < saved[RSP] - 128 && frame > saved[RSP] - 1024);
    assert_eq!(uc_stack(&buf), (ALT.0, 0, ALT.1));
}

#[test]
fn a_signal_on_the_alternate_stack_nests_below_it_there() {
    let mut saved = regs();
    /* Already running a handler there, so the flags read SS_ONSTACK. */
    saved[RSP] = ALT.0 + 0x6000;
    let (frame, buf, _) = build(&saved, 1, 2, 3, 0, Some(ALT), true).expect("frame");
    assert!(frame < saved[RSP] - 128 && frame > ALT.0);
    assert_eq!(uc_stack(&buf), (ALT.0, 1, ALT.1));
}

#[test]
fn a_frame_that_would_run_off_the_alternate_stack_is_refused() {
    let mut saved = regs();
    saved[RSP] = ALT.0 + 0x200; /* too near the base for another frame */
    assert!(build(&saved, 1, 2, 3, 0, Some(ALT), true).is_none());
}

#[test]
fn no_alternate_stack_reads_back_disabled() {
    let (_, buf, _) = build(&regs(), 1, 2, 3, 0, None, true).expect("frame");
    assert_eq!(uc_stack(&buf), (0, 2, 0)); /* SS_DISABLE */
}
