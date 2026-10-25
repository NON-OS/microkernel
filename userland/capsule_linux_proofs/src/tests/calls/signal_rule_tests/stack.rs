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

//! Where a thread is on its stack, what sigaltstack reports, and random stacks.

use super::super::super::random::Regs;
use super::super::fake_memory::Fake;
use crate::linux::call::signal_act::signal_of;
use crate::linux::guest::sigalt::call::stack_t_bytes;
use crate::linux::guest::sigalt::stack::{
    report, MINSIGSTKSZ, SS_AUTODISARM, SS_DISABLE, SS_ONSTACK,
};
use crate::linux::guest::sigalt::{change, on_stack, sigaltstack, NONE};
use crate::linux::guest::sigrestart::after;

pub(super) const SP: u64 = 0x10_0000;
pub(super) const SIZE: u64 = 0x2000;
pub(super) const SET: [u64; 3] = [SP, 0, SIZE];

/// The stack held, rsp, the stack asked for, and what sigaltstack leaves.
pub(super) type StackRow = ([u64; 3], u64, [u64; 3], Result<[u64; 3], i64>);

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
