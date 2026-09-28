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

use crate::sigframe::{build, returned, SIGCONTEXT_OFF, WORDS};

const RSP: usize = 15;
const RAX: usize = 13;

fn regs() -> [u64; WORDS] {
    let mut r = [0u64; WORDS];
    for (i, w) in r.iter_mut().enumerate() {
        *w = 0x1111_0000 + i as u64; // a distinct value per register
    }
    r[RSP] = 0x7fff_ffe0_0000; // a plausible stack pointer, page aligned
    r
}

#[test]
fn a_returning_frame_restores_the_registers_the_handler_was_entered_over() {
    let saved = regs();
    let (frame, buf, enter) =
        build(&saved, 0xdead_beef, 0xca11, 11, 0x1234, None, false).expect("frame");
    // The handler is entered at the frame, below the old stack, 16-byte down 8.
    assert!(frame < saved[RSP] - 128);
    assert_eq!(enter[RSP], frame);
    assert_eq!(enter[16], 0xdead_beef); // rip = handler
    assert_eq!(enter[8], 11); // rdi = signum
    assert_eq!(enter[12], frame + 8); // rdx = &ucontext
                                      // rt_sigreturn reads the ucontext the guest's rsp points at: frame + 8.
    let uc = &buf[8..];
    assert_eq!(returned(uc).unwrap(), saved);
}

#[test]
fn the_syscall_return_value_rides_in_the_saved_rax() {
    let mut saved = regs();
    saved[RAX] = 0; // as the thread trapped, before we set the reply
    saved[RAX] = 42; // the value the interrupted syscall returns
    let (_, buf, _) = build(&saved, 1, 2, 3, 0, None, false).expect("frame");
    assert_eq!(returned(&buf[8..]).unwrap()[RAX], 42);
}

#[test]
fn a_stack_too_low_to_hold_a_frame_is_refused() {
    let mut low = regs();
    low[RSP] = 64; // below the red zone plus a frame
    assert!(build(&low, 1, 2, 3, 0, None, false).is_none());
}

#[test]
fn the_sigcontext_sits_where_the_ucontext_says() {
    // uc_mcontext is at SIGCONTEXT_OFF within the ucontext, which is at frame+8.
    let saved = regs();
    let (_, buf, _) = build(&saved, 1, 2, 3, 0, None, false).expect("frame");
    let at = 8 + SIGCONTEXT_OFF;
    let r8 = u64::from_le_bytes(buf[at..at + 8].try_into().unwrap());
    assert_eq!(r8, saved[0]);
}

#[test]
fn the_registers_and_mask_sit_where_linux_programs_read_them() {
    // Offsets within the ucontext, from musl's and glibc's own
    // offsetof(ucontext_t, ...) on x86-64: uc_mcontext.gregs[REG_R8] at 40,
    // REG_RSP at 160, REG_RIP at 168, uc_sigmask at 296. The ucontext is at
    // frame + 8, after the return address.
    let saved = regs();
    let (_, buf, _) = build(&saved, 1, 2, 3, 0x0000_8000_0000_0001, None, false).expect("frame");
    let word = |at: usize| u64::from_le_bytes(buf[8 + at..8 + at + 8].try_into().unwrap());
    assert_eq!(word(40), saved[0]); // r8
    assert_eq!(word(160), saved[RSP]);
    assert_eq!(word(168), saved[16]); // rip
    assert_eq!(word(296), 0x0000_8000_0000_0001); // the mask
}

/// A 32 KiB alternate stack well away from the thread's own stack.
const ALT: (u64, u64) = (0x7000_0000_0000, 0x8000);

fn uc_stack(buf: &[u8]) -> (u64, u64, u64) {
    let word = |at: usize| u64::from_le_bytes(buf[8 + at..8 + at + 8].try_into().unwrap());
    (word(16), word(24) & 0xffff_ffff, word(32)) // ss_sp, ss_flags, ss_size
}

#[test]
fn an_onstack_handler_is_entered_at_the_top_of_the_alternate_stack() {
    let saved = regs();
    let (frame, buf, enter) = build(&saved, 1, 2, 3, 0, Some(ALT), true).expect("frame");
    let (sp, size) = ALT;
    assert!(frame > sp && frame < sp + size, "frame {frame:#x} is on the alternate stack");
    assert!(sp + size - frame < 512, "and at its top");
    assert_eq!(enter[RSP], frame);
    // The thread's own rsp is what rt_sigreturn gives back.
    assert_eq!(returned(&buf[8..]).unwrap(), saved);
    // uc_stack names the alternate stack; the thread was not on it.
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
    saved[RSP] = ALT.0 + 0x6000; // already running a handler there
    let (frame, buf, _) = build(&saved, 1, 2, 3, 0, Some(ALT), true).expect("frame");
    assert!(frame < saved[RSP] - 128 && frame > ALT.0);
    assert_eq!(uc_stack(&buf), (ALT.0, 1, ALT.1)); // SS_ONSTACK
}

#[test]
fn a_frame_that_would_run_off_the_alternate_stack_is_refused() {
    let mut saved = regs();
    saved[RSP] = ALT.0 + 0x200; // too near the base for another frame
    assert!(build(&saved, 1, 2, 3, 0, Some(ALT), true).is_none());
}

#[test]
fn no_alternate_stack_reads_back_disabled() {
    let (_, buf, _) = build(&regs(), 1, 2, 3, 0, None, true).expect("frame");
    assert_eq!(uc_stack(&buf), (0, 2, 0)); // SS_DISABLE
}
