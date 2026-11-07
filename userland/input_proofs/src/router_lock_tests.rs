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

//! The locked session (route/session_lock.rs): while the shell's lock
//! screen is up no window hears a key go down, a button, a wheel step or a
//! motion, whatever has focus or holds a grab, with or without the trusted
//! hold; a key held when the lock came up is let go where it went down; only
//! the shell locks and unlocks; Super+L reaches the shell over any focus.

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP,
    INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL,
};

use crate::clients::world;
use crate::route::chord::is_lock_chord;
use crate::route::session_lock::{
    accept, after, key_target, parse, pointer_shown, word, Word, MAX_SHOWN,
};
use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;
const KEY_X: u32 = b'x' as u32;
const KEY_L: u32 = b'l' as u32;
const META: u16 = 1 << 3;
const SHIFT: u16 = 1;

fn locked_desk() -> crate::state::Context {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    window(&mut ctx, B, (400, 0, 400, 400));
    world::set_focus(A);
    ctx.session_locked = true;
    ctx
}

fn every_kind_of_input(ctx: &mut crate::state::Context) {
    key_down(ctx, KEY_X, 0);
    key_up(ctx, KEY_X, 0);
    key_down(ctx, b'\n' as u32, MOD_CTRL | MOD_ALT);
    key_up(ctx, b'\n' as u32, MOD_CTRL | MOD_ALT);
    point(ctx, 100, 100);
    down(ctx, LEFT);
    up(ctx, LEFT);
    point(ctx, 500, 100);
    down(ctx, RIGHT);
    up(ctx, RIGHT);
    wheel(ctx, -2);
}

#[test]
fn while_locked_no_window_hears_anything() {
    let mut ctx = locked_desk();
    every_kind_of_input(&mut ctx);
    let rx = sent();
    assert!(rx.iter().all(|r| r.pid == SHELL), "a window heard input while locked: {rx:?}");
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN).len(), 2);
    assert_eq!(of_kind(&rx, INPUT_KIND_BUTTON_DOWN), [(SHELL, LEFT), (SHELL, RIGHT)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_BUTTON_UP), [(SHELL, LEFT), (SHELL, RIGHT)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_WHEEL).len(), 1);
    assert!(!of_kind(&rx, INPUT_KIND_POINTER_REL).is_empty());
}

#[test]
fn a_grab_held_by_a_window_takes_nothing_while_locked() {
    let mut ctx = locked_desk();
    assert!(ctx.grabs.request(A, 0xFFFF_FFFF), "a grab taken before the lock");
    every_kind_of_input(&mut ctx);
    assert!(sent().iter().all(|r| r.pid == SHELL), "the grab's holder heard input while locked");
}

#[test]
fn the_lock_does_not_lapse_with_the_trusted_hold() {
    // No trusted hold at all: the compositor stopped telling, or never did.
    let mut ctx = locked_desk();
    assert!(ctx.trusted.is_none());
    for _ in 0..50 {
        key_down(&mut ctx, KEY_X, 0);
        key_up(&mut ctx, KEY_X, 0);
    }
    assert!(sent().iter().all(|r| r.pid == SHELL));
}

#[test]
fn every_chord_goes_to_the_shell_while_locked() {
    let mut ctx = locked_desk();
    key_down(&mut ctx, 0x1B, MOD_CTRL | MOD_ALT);
    key_down(&mut ctx, b't' as u32, MOD_CTRL | MOD_ALT);
    key_down(&mut ctx, 0x09, MOD_ALT);
    key_down(&mut ctx, KEY_L, META);
    assert!(sent().iter().all(|r| r.pid == SHELL));
}

#[test]
fn a_key_held_when_the_lock_came_up_is_let_go_where_it_went_down() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, KEY_X, 0);
    ctx.session_locked = true;
    key_up(&mut ctx, KEY_X, 0);
    key_down(&mut ctx, KEY_X, 0);
    key_up(&mut ctx, KEY_X, 0);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(A, KEY_X), (SHELL, KEY_X)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(A, KEY_X), (SHELL, KEY_X)]);
}

#[test]
fn with_no_shell_running_input_goes_nowhere() {
    let mut ctx = locked_desk();
    world::with(|w| w.shell = 0);
    ctx.shell_pid = 0;
    every_kind_of_input(&mut ctx);
    assert!(sent().is_empty(), "input reached a window while locked with no shell");
}

#[test]
fn unlocking_gives_the_focused_window_its_input_back() {
    let mut ctx = locked_desk();
    key_down(&mut ctx, KEY_X, 0);
    key_up(&mut ctx, KEY_X, 0);
    ctx.session_locked = after(ctx.session_locked, Word::Unlock);
    key_down(&mut ctx, KEY_X, 0);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, KEY_X), (A, KEY_X)]);
}

#[test]
fn only_the_shell_sets_the_lock_and_the_words_are_three() {
    assert!(accept(SHELL, Some(SHELL)));
    assert!(!accept(A, Some(SHELL)));
    assert!(!accept(0, Some(0)));
    assert!(!accept(SHELL, None));
    assert_eq!(word(0), Some(Word::Unlock));
    assert_eq!(word(1), Some(Word::Lock));
    assert_eq!(word(2), Some(Word::Ask));
    assert_eq!(word(3), None);
    assert!(after(false, Word::Lock));
    assert!(after(true, Word::Ask));
    assert!(!after(false, Word::Ask));
    assert!(!after(true, Word::Unlock));
}

#[test]
fn a_release_follows_its_press_and_a_press_goes_to_the_shell() {
    assert_eq!(key_target(true, Some(A), SHELL), A);
    assert_eq!(key_target(false, Some(A), SHELL), SHELL);
    assert_eq!(key_target(true, None, SHELL), SHELL);
    assert_eq!(key_target(true, Some(0), SHELL), SHELL);
}

#[test]
fn super_l_alone_is_the_lock_chord() {
    assert!(is_lock_chord(INPUT_KIND_KEY_DOWN, KEY_L, META));
    assert!(is_lock_chord(INPUT_KIND_KEY_DOWN, b'L' as u32, META));
    assert!(!is_lock_chord(INPUT_KIND_KEY_UP, KEY_L, META));
    assert!(!is_lock_chord(INPUT_KIND_KEY_DOWN, KEY_L, 0));
    assert!(!is_lock_chord(INPUT_KIND_KEY_DOWN, KEY_L, META | MOD_CTRL));
    assert!(!is_lock_chord(INPUT_KIND_KEY_DOWN, KEY_L, META | MOD_ALT));
    assert!(!is_lock_chord(INPUT_KIND_KEY_DOWN, KEY_L, META | SHIFT));
    assert!(!is_lock_chord(INPUT_KIND_KEY_DOWN, b'k' as u32, META));
}

#[test]
fn super_l_reaches_the_shell_over_a_focused_window() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, KEY_L, META);
    key_up(&mut ctx, KEY_L, META);
    key_down(&mut ctx, KEY_L, 0);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, KEY_L), (A, KEY_L)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, KEY_L)]);
}

#[test]
fn the_lock_op_is_0x0008_and_the_router_serves_it() {
    assert_eq!(crate::router_protocol::OP_SESSION_LOCK, 0x0008);
    let drain = include_str!("../../capsule_input_router/src/server/drain_ipc.rs");
    assert!(drain.contains("OP_SESSION_LOCK => handlers::session_lock::handle("));
    let handler = include_str!("../../capsule_input_router/src/server/handlers/session_lock.rs");
    assert!(handler.contains("if !accept(sender_pid, lookup_pid(SHELL))"), "the shell only");
    assert!(handler.contains("const SHELL: &[u8] = b\"desktop_shell\";"));
    let dispatch = include_str!("../../capsule_input_router/src/route/dispatch.rs");
    let locked = dispatch.find("if ctx.session_locked || posted_locked {").expect("dispatch checks the lock");
    let grab = dispatch.find("ctx.grabs.holder_for").expect("grab routing");
    let held = dispatch.find("Route::Held(h) if key.is_some()").expect("hold routing");
    assert!(locked < grab && locked < held, "the lock is asked before any grab or hold");
}

fn body(w: u32, rects: &[(u32, u32, u32, u32)]) -> Vec<u8> {
    let mut b = w.to_le_bytes().to_vec();
    for r in rects {
        for v in [r.0, r.1, r.2, r.3] {
            b.extend_from_slice(&v.to_le_bytes());
        }
    }
    b
}

#[test]
fn a_lock_word_carries_up_to_four_rectangles() {
    let field = (300, 280, 200, 40);
    let (w, shown) = parse(&body(1, &[field])).unwrap();
    assert_eq!(w, Word::Lock);
    assert_eq!(shown, [Some(field), None, None, None]);
    let four = [field; MAX_SHOWN];
    assert!(parse(&body(1, &four)).is_some());
    assert!(parse(&body(1, &[field; 5])).is_none());
    assert_eq!(parse(&body(2, &[])).unwrap().0, Word::Ask);
    assert!(parse(&body(9, &[])).is_none());
    let mut ragged = body(1, &[field]);
    ragged.pop();
    assert!(parse(&ragged).is_none());
    assert!(parse(&[1, 0]).is_none());
}

#[test]
fn while_locked_the_pointer_shows_over_the_fields_only() {
    let shown = [Some((300, 280, 200, 40)), Some((1100, 280, 200, 40)), None, None];
    assert!(pointer_shown(true, &shown, 300, 280));
    assert!(pointer_shown(true, &shown, 499, 319));
    assert!(pointer_shown(true, &shown, 1150, 300));
    assert!(!pointer_shown(true, &shown, 500, 300));
    assert!(!pointer_shown(true, &shown, 299, 300));
    assert!(!pointer_shown(true, &shown, 10, 10));
    assert!(!pointer_shown(true, &[None; MAX_SHOWN], 300, 280));
    // Unlocked, it shows everywhere, whatever rectangles were last told.
    assert!(pointer_shown(false, &shown, 10, 10));
}

#[test]
fn the_router_sends_the_pointers_visibility_from_the_lock() {
    let runner = include_str!("../../capsule_input_router/src/server/runner.rs");
    assert!(runner.contains("crate::route::session_lock::pointer_shown("));
    assert!(
        runner.contains("compositor::cursor_update(&mut ctx.compositor_port, rid, x, y, shown)")
    );
}
