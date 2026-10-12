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

//! The input ring as the driver sees it: a post that always succeeds and
//! lands, in order, in a per-thread list.

use std::cell::RefCell;

pub const INPUT_KIND_KEY_DOWN: u16 = 0;
pub const INPUT_KIND_KEY_UP: u16 = 1;
pub const INPUT_KIND_POINTER_REL: u16 = 2;
pub const INPUT_KIND_POINTER_ABS: u16 = 3;
pub const INPUT_KIND_WHEEL: u16 = 4;
pub const INPUT_KIND_BUTTON_DOWN: u16 = 5;
pub const INPUT_KIND_BUTTON_UP: u16 = 6;

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

thread_local! {
    static POSTED: RefCell<Vec<InputEvent>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_input_event_post(ev: *const InputEvent) -> i64 {
    // SAFETY: the driver posts a pointer to an event of its own, live and
    // aligned for the call.
    let ev = unsafe { *ev };
    POSTED.with(|p| p.borrow_mut().push(ev));
    0
}

/// Every event posted since the last take, in order.
pub fn take_posted() -> Vec<InputEvent> {
    POSTED.with(|p| std::mem::take(&mut *p.borrow_mut()))
}
