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

//! What sigaltstack refuses, reads and writes.

use super::super::fake_memory::Fake;
use super::stack::{StackRow, SET, SIZE, SP};
use crate::linux::abi::errno::{fail, ok, EFAULT, EINVAL, ENOMEM, EPERM};
use crate::linux::guest::sigalt::call::{stack_t, stack_t_bytes};
use crate::linux::guest::sigalt::stack::{MINSIGSTKSZ, SS_AUTODISARM, SS_DISABLE, SS_ONSTACK};
use crate::linux::guest::sigalt::{change, sigaltstack, NONE};

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
