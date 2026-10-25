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

//! The router's trusted-path rules on their own: only the compositor's
//! notice is believed, a hold lapses when it is not heard again, and what
//! goes ahead of grabs and focus, case by case.

use crate::route::preempt::{preempt, Key, Route};
use crate::route::trusted_hold::{accept, holder, told, HOLD_LEASE_MS};
use crate::router_protocol::OP_TRUSTED_HOLD;

const SHELL: u32 = 10;
const APP: u32 = 20;

#[test]
fn only_the_compositors_notice_is_believed_and_a_hold_lapses() {
    assert_eq!(OP_TRUSTED_HOLD, 0x0005);
    assert!(accept(5, Some(5)));
    assert!(!accept(6, Some(5)), "another process");
    assert!(!accept(5, None), "no compositor running");
    assert!(!accept(0, Some(0)));
    assert_eq!(told(0, 100), None, "0 ends the hold");
    let h = told(SHELL, 100);
    assert_eq!(holder(h, 100), Some(SHELL));
    assert_eq!(holder(h, 100 + HOLD_LEASE_MS - 1), Some(SHELL));
    assert_eq!(holder(h, 100 + HOLD_LEASE_MS), None, "not heard again: lapsed");
    assert_eq!(holder(told(SHELL, i64::MAX), i64::MAX - 1), Some(SHELL), "no overflow");
}

fn key(down: bool, reserved: bool, pressed_in: Option<u32>) -> Option<Key> {
    Some(Key { down, reserved, pressed_in })
}

#[test]
fn what_goes_ahead_of_grabs_and_focus() {
    let held = Some(SHELL);
    // A reserved press goes to the shell, held or not; with no shell, as before.
    assert_eq!(preempt(key(true, true, None), None, SHELL), Route::To(SHELL));
    assert_eq!(preempt(key(true, true, None), Some(77), SHELL), Route::To(SHELL));
    assert_eq!(preempt(key(true, true, None), None, 0), Route::Normal);
    // A release follows a press that went to the shell, or any press while held.
    assert_eq!(preempt(key(false, false, Some(SHELL)), None, SHELL), Route::To(SHELL));
    assert_eq!(preempt(key(false, false, Some(APP)), held, SHELL), Route::To(APP));
    assert_eq!(preempt(key(false, false, Some(APP)), None, SHELL), Route::Normal);
    assert_eq!(preempt(key(false, false, Some(0)), held, SHELL), Route::Held(SHELL));
    // While held, everything else goes to the holder, keys and pointer alike.
    assert_eq!(preempt(key(true, false, None), held, SHELL), Route::Held(SHELL));
    assert_eq!(preempt(None, held, SHELL), Route::Held(SHELL));
    assert_eq!(preempt(None, None, SHELL), Route::Normal);
    assert_eq!(preempt(key(true, false, None), None, SHELL), Route::Normal);
}
