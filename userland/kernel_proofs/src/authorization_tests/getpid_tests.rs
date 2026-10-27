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
//! A capsule may always learn its own pid. MkGetPid needs only a valid
//! token, as MkProcStat does, which shows every pid to any valid caller.
//! CoreExec still gates the rest: whatever a token holds, CoreExec adds its
//! spawn args, and never its pid any more.

use super::token::{registered_syscalls, token};
use crate::capabilities::Capability;
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn a_display_driver_s_token_may_ask_its_own_pid() {
    // driver.virtio_gpu0 holds hardware and IPC caps, and no CoreExec.
    let driver = token(vec![Capability::Hardware, Capability::IPC]);
    assert!(is_allowed(&driver, SyscallNumber::MkGetPid));
    assert!(!is_allowed(&driver, SyscallNumber::MkArgs), "args stay behind CoreExec");
    assert!(!is_allowed(&token(vec![]), SyscallNumber::MkGetPid), "an empty token is not valid");
}

#[test]
fn core_exec_adds_args_and_never_getpid() {
    let bases = [
        vec![Capability::Hardware],
        vec![Capability::IPC, Capability::Memory],
        vec![Capability::Hardware, Capability::IPC, Capability::Debug],
    ];
    for base in bases {
        let without = token(base.clone());
        let mut caps = base;
        caps.push(Capability::CoreExec);
        let with = token(caps);
        let added: Vec<SyscallNumber> = registered_syscalls()
            .into_iter()
            .filter(|&n| is_allowed(&with, n) && !is_allowed(&without, n))
            .collect();
        assert!(added.contains(&SyscallNumber::MkArgs), "{added:?}");
        assert!(!added.contains(&SyscallNumber::MkGetPid), "{added:?}");
    }
}
