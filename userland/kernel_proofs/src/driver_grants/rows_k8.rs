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

//! the raw input ring's gates, on the real cap table. Posting takes
//! InputSource alone; draining and waiting take InputConsumer alone; Irq,
//! which every driver holds, and Admin stand in for neither.

extern crate alloc;
use alloc::vec::Vec;

use super::token::token;
use crate::capabilities::Capability;
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber;

const ROWS: [(SyscallNumber, Capability); 3] = [
    (SyscallNumber::MkInputEventPost, Capability::InputSource),
    (SyscallNumber::MkInputEventDrain, Capability::InputConsumer),
    (SyscallNumber::MkInputEventWait, Capability::InputConsumer),
];

#[test]
fn each_input_call_takes_its_one_right_and_no_other() {
    for (call, cap) in ROWS {
        assert!(is_allowed(&token(alloc::vec![cap]), call), "{call:?} with {cap:?}");
        for other in Capability::all().iter().copied().filter(|c| *c != cap) {
            assert!(!is_allowed(&token(alloc::vec![other]), call), "{call:?} with {other:?}");
        }
        let all_but: Vec<Capability> =
            Capability::all().iter().copied().filter(|c| *c != cap).collect();
        assert!(!is_allowed(&token(all_but), call), "{call:?} with every bit but {cap:?}");
    }
}

#[test]
fn a_driver_cannot_drain_keys_nor_the_router_post_them() {
    let driver = token(alloc::vec![Capability::Irq, Capability::InputSource, Capability::Admin]);
    assert!(is_allowed(&driver, SyscallNumber::MkInputEventPost));
    assert!(!is_allowed(&driver, SyscallNumber::MkInputEventDrain), "drain");
    assert!(!is_allowed(&driver, SyscallNumber::MkInputEventWait), "wait");
    let router = token(alloc::vec![Capability::IPC, Capability::Memory, Capability::InputConsumer]);
    assert!(is_allowed(&router, SyscallNumber::MkInputEventDrain));
    assert!(!is_allowed(&router, SyscallNumber::MkInputEventPost), "the router never posts");
}

/// D2: ps2_input posts each key itself, under its spawn grant
/// (src/hardware/ps2_kbd_capsule/spawn.rs), and still drains nothing.
#[test]
fn ps2_input_posts_under_its_own_grant() {
    use Capability::*;
    let ps2 = token(alloc::vec![IPC, Memory, Driver, DeviceEnum, Pio, Irq, InputSource]);
    assert!(is_allowed(&ps2, SyscallNumber::MkInputEventPost));
    assert!(!is_allowed(&ps2, SyscallNumber::MkInputEventDrain));
    assert!(!is_allowed(&ps2, SyscallNumber::MkInputEventWait));
}
