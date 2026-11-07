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

//! Input posted while the lock screen was up and drained after the unlock
//! (route/session_lock.rs posted_locked). The router serves the shell's
//! unlock before it drains the ring, so a key typed while the passphrase was
//! checked, with the router not run until the check was done, was routed as
//! if typed after the unlock and reached the focused window. It is routed by
//! the time the kernel dated it, as the lock then sent it: to the shell.

use nonos_libc::{
    advance_ms, mk_uptime_ms, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_KEY_DOWN,
    INPUT_KIND_KEY_UP,
};

use crate::clients::world;
use crate::route::route_event;
use crate::route::session_lock::{posted_locked, unlocked_at, Word};
use crate::router_desk::*;

const A: u32 = 20;
const KEY_X: u32 = b'x' as u32;
const NS_PER_MS: u64 = 1_000_000;

fn dated(kind: u16, code: u32, at_ms: i64) -> InputEvent {
    let timestamp_ns = at_ms as u64 * NS_PER_MS;
    InputEvent { kind, flags: 0, code, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns }
}

/// A desk with A focused, locked at 1000 ms and unlocked at 3000 ms, as the
/// router's handler of the shell's words would leave it.
fn unlocked_after_a_check() -> crate::state::Context {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    advance_ms(1000);
    ctx.unlocked_ms = unlocked_at(ctx.session_locked, Word::Lock, mk_uptime_ms(), None);
    ctx.session_locked = true;
    advance_ms(2000);
    ctx.unlocked_ms = unlocked_at(ctx.session_locked, Word::Unlock, mk_uptime_ms(), None);
    ctx.session_locked = false;
    ctx
}

#[test]
fn keys_typed_during_the_check_and_drained_after_the_unlock_go_to_the_shell() {
    let mut ctx = unlocked_after_a_check();
    for at in [1500, 2999, 3000] {
        route_event(&mut ctx, &dated(INPUT_KIND_KEY_DOWN, KEY_X, at));
        route_event(&mut ctx, &dated(INPUT_KIND_KEY_UP, KEY_X, at));
    }
    route_event(&mut ctx, &dated(INPUT_KIND_BUTTON_DOWN, LEFT, 2500));
    let rx = sent();
    assert!(rx.iter().all(|r| r.pid == SHELL), "a window heard input typed while locked: {rx:?}");
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN).len(), 3);
}

#[test]
fn a_key_posted_after_the_unlock_reaches_the_focused_window() {
    let mut ctx = unlocked_after_a_check();
    route_event(&mut ctx, &dated(INPUT_KIND_KEY_DOWN, KEY_X, 2000));
    route_event(&mut ctx, &dated(INPUT_KIND_KEY_DOWN, KEY_X, 3001));
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, KEY_X), (A, KEY_X)]);
}

#[test]
fn only_an_unlock_that_ends_a_lock_moves_the_instant() {
    assert_eq!(unlocked_at(true, Word::Unlock, 3000, None), Some(3000));
    assert_eq!(unlocked_at(true, Word::Unlock, 3000, Some(10)), Some(3000));
    assert_eq!(unlocked_at(false, Word::Unlock, 5000, Some(3000)), Some(3000));
    assert_eq!(unlocked_at(false, Word::Lock, 5000, Some(3000)), Some(3000));
    assert_eq!(unlocked_at(true, Word::Ask, 5000, Some(3000)), Some(3000));
    assert_eq!(unlocked_at(false, Word::Unlock, 5000, None), None);
    assert_eq!(unlocked_at(true, Word::Unlock, -1, Some(7)), Some(7));
}

#[test]
fn an_event_is_locked_by_its_date_and_an_undated_one_is_taken_as_now() {
    assert!(!posted_locked(None, 5 * NS_PER_MS));
    assert!(posted_locked(Some(3000), 3000 * NS_PER_MS));
    assert!(posted_locked(Some(3000), 3000 * NS_PER_MS + NS_PER_MS - 1));
    assert!(!posted_locked(Some(3000), 3001 * NS_PER_MS));
    assert!(!posted_locked(Some(3000), 0));
    assert!(posted_locked(Some(u64::MAX), u64::MAX));
}

#[test]
fn the_handler_keeps_the_instant_before_it_sets_the_lock() {
    let handler = include_str!("../../capsule_input_router/src/server/handlers/session_lock.rs");
    let instant = handler.find("ctx.unlocked_ms = unlocked_at(ctx.session_locked, w,");
    let lock = handler.find("ctx.session_locked = after(ctx.session_locked, w);");
    assert!(instant.expect("the handler keeps the instant") < lock.expect("and sets the lock"));
}
