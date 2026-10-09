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

//! The things the included input router files take from `nonos_libc`: the
//! input event, a send to a pid, whether a pid is alive and the clock. A send
//! lands in a per-thread outbox the tests read back; a pid can be marked dead,
//! or alive with a full inbox, to hold the router to what it does when a send
//! fails.

use std::cell::RefCell;
use std::collections::HashSet;

pub const INPUT_KIND_KEY_DOWN: u16 = 0;
pub const INPUT_KIND_KEY_UP: u16 = 1;
pub const INPUT_KIND_POINTER_REL: u16 = 2;
pub const INPUT_KIND_POINTER_ABS: u16 = 3;
pub const INPUT_KIND_WHEEL: u16 = 4;
pub const INPUT_KIND_BUTTON_DOWN: u16 = 5;
pub const INPUT_KIND_BUTTON_UP: u16 = 6;
pub const INPUT_KIND_TOUCH: u16 = 7;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct InputEvent {
    pub kind: u16,
    pub flags: u16,
    pub code: u32,
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
    pub timestamp_ns: u64,
}

#[derive(Default)]
struct Ipc {
    posted: Vec<InputEvent>,
    sent: Vec<(u32, Vec<u8>)>,
    dead: HashSet<u32>,
    full: HashSet<u32>,
    now_ms: i64,
}

thread_local! {
    static IPC: RefCell<Ipc> = RefCell::new(Ipc::default());
}

/// A send succeeds unless the pid is dead or its inbox is full.
pub fn mk_ipc_send_to_pid(dest_pid: u32, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the router passes the pointer and length of one of its own
    // frames, live for the call.
    let frame = unsafe { std::slice::from_raw_parts(buf, len) }.to_vec();
    IPC.with(|ipc| {
        let mut ipc = ipc.borrow_mut();
        if ipc.dead.contains(&dest_pid) || ipc.full.contains(&dest_pid) {
            return -1;
        }
        ipc.sent.push((dest_pid, frame));
        len as i64
    })
}

/// A driver's post to the kernel input ring, kept in order.
pub fn mk_input_event_post(ev: *const InputEvent) -> i64 {
    // SAFETY: drivers post a pointer to an event of their own, live for the
    // call.
    let ev = unsafe { *ev };
    IPC.with(|ipc| ipc.borrow_mut().posted.push(ev));
    0
}

/// Every event posted to the input ring since the last take, in order.
pub fn take_posted() -> Vec<InputEvent> {
    IPC.with(|ipc| std::mem::take(&mut ipc.borrow_mut().posted))
}

pub fn mk_pid_alive(pid: u32) -> bool {
    IPC.with(|ipc| !ipc.borrow().dead.contains(&pid))
}

pub fn mk_uptime_ms() -> i64 {
    IPC.with(|ipc| ipc.borrow().now_ms)
}

pub fn mk_time_millis() -> i64 {
    mk_uptime_ms()
}

/// Every frame sent since the last take, in order, with its destination.
pub fn take_sent() -> Vec<(u32, Vec<u8>)> {
    IPC.with(|ipc| std::mem::take(&mut ipc.borrow_mut().sent))
}

pub fn kill(pid: u32) {
    IPC.with(|ipc| {
        ipc.borrow_mut().dead.insert(pid);
    });
}

pub fn set_full(pid: u32, full: bool) {
    IPC.with(|ipc| {
        let mut ipc = ipc.borrow_mut();
        if full {
            ipc.full.insert(pid);
        } else {
            ipc.full.remove(&pid);
        }
    });
}

pub fn advance_ms(ms: i64) {
    IPC.with(|ipc| ipc.borrow_mut().now_ms += ms);
}

/// A fresh thread: nothing sent, every pid alive, the clock at zero.
pub fn reset() {
    IPC.with(|ipc| *ipc.borrow_mut() = Ipc::default());
}
