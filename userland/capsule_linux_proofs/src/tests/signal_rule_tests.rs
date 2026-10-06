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

//! The signal calls' rules: sigaltstack as do_sigaltstack keeps it, with
//! SS_AUTODISARM; what an interrupted wait answers under SA_RESTART; and the
//! signals rt_sigaction may name and the mask it stores.

use super::fake_memory::Fake;
use super::random::Regs;
use crate::linux::abi::errno::{fail, ok, EFAULT, EINTR, EINVAL, ENOMEM, EPERM};
use crate::linux::call::signal_act::{decode, encode, signal_of, wait_mask, wait_set};
use crate::linux::guest::sigalt::{
    change, entered, on_stack, report, sigaltstack, stack_t, stack_t_bytes, MINSIGSTKSZ, NONE,
    SS_AUTODISARM, SS_DISABLE, SS_ONSTACK,
};
use crate::linux::guest::sigrestart::{after, After};
use crate::linux::guest::sigstate::{SigAction, SA_RESTART};

const SP: u64 = 0x10_0000;
const SIZE: u64 = 0x2000;
const SET: [u64; 3] = [SP, 0, SIZE];

/// The stack held, rsp, the stack asked for, and what sigaltstack leaves.
type StackRow = ([u64; 3], u64, [u64; 3], Result<[u64; 3], i64>);

#[test]
fn a_thread_is_on_its_stack_above_the_base_and_up_to_the_top() {
    assert!(!on_stack(SET, SP), "the base itself is below the stack");
    assert!(on_stack(SET, SP + 1));
    assert!(on_stack(SET, SP + SIZE), "the top is where a frame starts");
    assert!(!on_stack(SET, SP + SIZE + 1));
    assert!(!on_stack(NONE, 8));
    assert!(!on_stack([SP, SS_AUTODISARM, SIZE], SP + 8), "an autodisarm stack never counts");
}

#[test]
fn sigaltstack_reports_disabled_on_or_neither_and_keeps_autodisarm() {
    assert_eq!(report(NONE, SP), [0, SS_DISABLE, 0]);
    assert_eq!(report(SET, SP + 8), [SP, SS_ONSTACK, SIZE]);
    assert_eq!(report(SET, 0x9000_0000), [SP, 0, SIZE]);
    assert_eq!(report([SP, SS_AUTODISARM, SIZE], SP + 8), [SP, SS_AUTODISARM, SIZE]);
    // A stack set with SS_ONSTACK reports what is true, not what was set.
    assert_eq!(report([SP, SS_ONSTACK, SIZE], 0), [SP, 0, SIZE]);
}

#[test]
fn sigaltstack_refuses_as_linux_does() {
    let on = SP + 16;
    let table: [StackRow; 9] = [
        (SET, on, [SP, 0, SIZE], Err(EPERM)),
        (SET, on, [0, SS_DISABLE, 0], Err(EPERM)),
        // SS_ONSTACK with SS_DISABLE is no mode at all.
        (NONE, 0, [SP, SS_ONSTACK | SS_DISABLE, SIZE], Err(EINVAL)),
        (NONE, 0, [SP, 4, SIZE], Err(EINVAL)),
        (NONE, 0, [SP, 0, MINSIGSTKSZ - 1], Err(ENOMEM)),
        (NONE, 0, [SP, 0, MINSIGSTKSZ], Ok([SP, 0, MINSIGSTKSZ])),
        (NONE, 0, [SP, SS_ONSTACK, SIZE], Ok([SP, SS_ONSTACK, SIZE])),
        (SET, 0, [7, SS_DISABLE, 9], Ok([0, SS_DISABLE, 0])),
        // A handler on an autodisarm stack may change it.
        ([SP, SS_AUTODISARM, SIZE], on, [0, SS_DISABLE, 0], Ok([0, SS_DISABLE, 0])),
    ];
    for (alt, rsp, new, want) in table {
        assert_eq!(change(alt, rsp, new), want, "{alt:x?} {rsp:#x} {new:x?}");
    }
}

#[test]
fn entering_a_handler_disarms_only_an_autodisarm_stack() {
    let armed = [SP, SS_AUTODISARM, SIZE];
    assert_eq!(entered(armed), NONE);
    assert_eq!(entered(SET), SET);
    // The frame records it armed, and its return puts it back.
    assert_eq!(change(entered(armed), SP + 64, armed), Ok(armed));
}

#[test]
fn sigaltstack_reads_the_new_stack_before_it_writes_the_old_into_the_same_buffer() {
    let mem = Fake::new(0x4000, 0x100);
    let buf = 0x4000;
    mem.put(buf, &stack_t_bytes([SP, 0, SIZE]));
    let (now, rc) = sigaltstack(&mem, NONE, 0, buf, buf);
    assert_eq!(rc, ok(0));
    assert_eq!(now, [SP, 0, SIZE], "the stack the guest asked for is the one set");
    assert_eq!(stack_t(&mem.get(buf, 24)), Some([0, SS_DISABLE, 0]), "and it reads the old one");
}

#[test]
fn a_refused_or_faulting_sigaltstack_changes_and_writes_nothing() {
    let mem = Fake::new(0x4000, 0x100);
    let (ss, old) = (0x4000, 0x4040);
    mem.put(ss, &stack_t_bytes([SP, 0, 100]));
    assert_eq!(sigaltstack(&mem, SET, 0, ss, old), (SET, fail(ENOMEM)));
    assert_eq!(sigaltstack(&mem, SET, 0, 0x1000, old), (SET, fail(EFAULT)));
    assert_eq!(mem.writes.get(), 0);
    // An old pointer that faults is EFAULT, after the change, as Linux has it.
    mem.put(ss, &stack_t_bytes([SP, 0, SIZE]));
    assert_eq!(sigaltstack(&mem, NONE, 0, ss, 0x1000), ([SP, 0, SIZE], fail(EFAULT)));
    // No new stack only reports.
    assert_eq!(sigaltstack(&mem, SET, SP + 8, 0, old), (SET, ok(0)));
    assert_eq!(stack_t(&mem.get(old, 24)), Some([SP, SS_ONSTACK, SIZE]));
}

#[test]
fn an_interrupted_wait_answers_as_linux_handle_signal_does() {
    const R: u64 = SA_RESTART;
    let again = After::Again;
    let eintr = After::Answer(fail(EINTR));
    // (nr, sa_flags, futex timeout, moved, timed, answer)
    let table: [(u64, u64, u64, u64, bool, After); 15] = [
        (0, R, 0, 0, false, again),
        (0, 0, 0, 0, false, eintr),
        (61, R, 0, 0, false, again),
        (247, R, 0, 0, false, again),
        // The lock waits and the multi-message socket calls restart too.
        (72, R, 0, 0, false, again),
        (73, R, 0, 0, false, again),
        (299, R, 0, 0, false, again),
        (202, R, 0, 0, false, again),
        (202, R, 0x7000, 0, false, eintr),
        // poll, epoll_wait, nanosleep and rt_sigsuspend never restart.
        (7, R, 0, 0, false, eintr),
        (232, R, 0, 0, false, eintr),
        (35, R, 0, 0, false, eintr),
        (130, R, 0, 0, false, eintr),
        // A socket wait with SO_RCVTIMEO is EINTR, SA_RESTART or not.
        (45, R, 0, 0, true, eintr),
        // A write that moved something answers that, and never runs again.
        (1, R, 0, 4096, false, After::Answer(ok(4096))),
    ];
    for (nr, flags, timeout, moved, timed, want) in table {
        assert_eq!(after(nr, flags, timeout, moved, timed), want, "nr {nr}");
    }
}

#[test]
fn rt_sigaction_names_an_int_signal_and_never_changes_kill_or_stop() {
    assert_eq!(signal_of(0, false), Err(EINVAL));
    assert_eq!(signal_of(65, false), Err(EINVAL));
    assert_eq!(signal_of(u64::MAX, false), Err(EINVAL));
    assert_eq!(signal_of(1, true), Ok(1));
    assert_eq!(signal_of(64, true), Ok(64));
    assert_eq!(signal_of(9, true), Err(EINVAL));
    assert_eq!(signal_of(19, true), Err(EINVAL));
    assert_eq!(signal_of(9, false), Ok(9), "SIGKILL's action may be read");
    assert_eq!(signal_of((1 << 32) | 2, true), Ok(2), "the signal is an int");
}

#[test]
fn a_stored_action_never_blocks_kill_or_stop() {
    let a = SigAction { handler: 0x40_1000, flags: 0x0400_0000, restorer: 0x40_2000, mask: !0 };
    let back = decode(&encode(a));
    let want = !((1u64 << 8) | (1 << 18));
    assert_eq!(
        back.map(|b| (b.handler, b.flags, b.restorer, b.mask)),
        Some((a.handler, a.flags, a.restorer, want))
    );
    assert!(decode(&[0u8; 31]).is_none());
}

#[test]
fn a_waiting_call_s_mask_is_read_as_set_user_sigmask_reads_it() {
    let mem = Fake::new(0x4000, 0x40);
    mem.put(0x4000, &u64::MAX.to_le_bytes());
    assert_eq!(wait_mask(&mem, 0, 99), Ok(None), "no mask: the size is not looked at");
    assert_eq!(wait_mask(&mem, 0x4000, 16), Err(EINVAL));
    assert_eq!(wait_mask(&mem, 0x1000, 8), Err(EFAULT));
    assert_eq!(wait_mask(&mem, 0x403c, 8), Err(EFAULT), "half of it outside");
    let all_but = !((1u64 << 8) | (1 << 18));
    assert_eq!(wait_mask(&mem, 0x4000, 8), Ok(Some(all_but)), "never SIGKILL or SIGSTOP");
}

#[test]
fn sigtimedwait_never_takes_sigkill_or_sigstop() {
    let mem = Fake::new(0x4000, 0x10);
    mem.put(0x4000, &u64::MAX.to_le_bytes());
    let set = wait_set(&mem, 0x4000);
    assert_eq!(set.map(|s| s & (1 << 8)), Some(0), "SIGKILL ends the process, never a wait");
    assert_eq!(set.map(|s| s & (1 << 18)), Some(0), "SIGSTOP likewise");
    assert_eq!(set.map(|s| s.count_ones()), Some(62));
    assert_eq!(wait_set(&mem, 0x400c), None);
}

#[test]
fn random_stacks_and_pointers_never_panic_and_keep_the_invariants() {
    let mut r = Regs::new(0x7369_6761_6c74_7374);
    let mem = Fake::new(0x4000, 0x80);
    for _ in 0..100_000 {
        let alt = [r.arg(), r.arg() & 0xffff_ffff, r.arg()];
        let (rsp, new) = (r.arg(), [r.arg(), r.arg() & 0xffff_ffff, r.arg()]);
        if let Ok(now) = change(alt, rsp, new) {
            assert!(now[2] == 0 || now[2] >= MINSIGSTKSZ);
            assert!(now[2] != 0 || now[1] & SS_DISABLE != 0);
        }
        let shown = report(alt, rsp);
        assert!(shown[1] & SS_ONSTACK == 0 || alt[1] & SS_AUTODISARM == 0);
        mem.put(0x4000, &stack_t_bytes(new));
        let (ss, old) = (0x4000 + r.small(0x80), 0x4000 + r.small(0x80));
        let _ = sigaltstack(&mem, alt, rsp, if r.small(4) == 0 { r.arg() } else { ss }, old);
        let _ = after(r.small(400), r.arg(), r.arg(), r.small(3), r.small(2) == 0);
        let _ = signal_of(r.arg(), r.small(2) == 0);
    }
}
