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

//! Idle: the screen blanks after the policy's timeout with no input and no
//! inhibit, any input through the router wakes it, an inhibit holds back
//! the blank but never wakes the screen, and the compositor is told each
//! change once, a refused word again after a second.

use nonos_libc::{advance_ms, mk_uptime_ms};

use crate::router_desk::*;
use crate::state::idle::{Idle, MAX_INHIBITORS, RETRY_MS};

const MIN: i64 = 60_000;

fn minute_desk() -> crate::state::Context {
    let mut ctx = desk(800, 600);
    ctx.idle.set_timeout_minutes(1);
    ctx.idle.input(mk_uptime_ms());
    ctx
}

#[test]
fn no_timeout_never_blanks() {
    let mut idle = Idle::new(0);
    idle.set_timeout_minutes(0);
    assert!(!idle.step(24 * 60 * MIN));
}

#[test]
fn the_screen_blanks_once_the_timeout_passed_with_no_input() {
    let mut idle = Idle::new(1_000);
    idle.set_timeout_minutes(5);
    assert!(!idle.step(1_000 + 5 * MIN - 1));
    assert!(idle.step(1_000 + 5 * MIN));
    assert!(idle.step(1_000 + 50 * MIN), "it stays blank");
}

#[test]
fn any_input_through_the_router_wakes_it_and_starts_the_count_again() {
    for wake in 0..3 {
        let mut ctx = minute_desk();
        advance_ms(MIN);
        assert!(ctx.idle.step(mk_uptime_ms()), "blank");
        advance_ms(1);
        match wake {
            0 => key_down(&mut ctx, 30, 0),
            1 => point(&mut ctx, 10, 10),
            _ => wheel(&mut ctx, 1),
        }
        assert!(!ctx.idle.step(mk_uptime_ms()), "woken by input {wake}");
        advance_ms(MIN - 1);
        assert!(!ctx.idle.step(mk_uptime_ms()), "a whole timeout again");
        advance_ms(1);
        assert!(ctx.idle.step(mk_uptime_ms()));
    }
}

#[test]
fn input_keeps_it_awake() {
    let mut ctx = minute_desk();
    for _ in 0..10 {
        advance_ms(MIN / 2);
        key_down(&mut ctx, 30, 0);
        key_up(&mut ctx, 30, 0);
        assert!(!ctx.idle.step(mk_uptime_ms()));
    }
}

#[test]
fn an_inhibit_holds_back_the_blank_but_never_wakes_a_blank_screen() {
    let mut idle = Idle::new(0);
    idle.set_timeout_minutes(1);
    assert!(idle.inhibit(40, true));
    assert!(!idle.step(10 * MIN), "held awake");
    assert!(idle.inhibit(40, false));
    assert!(idle.step(10 * MIN), "let go long after the last input: blank at once");
    assert!(idle.inhibit(41, true));
    assert!(idle.step(11 * MIN), "an inhibit taken now does not wake it");
    idle.input(11 * MIN + 1);
    assert!(!idle.step(11 * MIN + 1), "input does");
}

#[test]
fn a_timeout_turned_off_wakes_a_blank_screen() {
    let mut idle = Idle::new(0);
    idle.set_timeout_minutes(1);
    assert!(idle.step(MIN));
    idle.set_timeout_minutes(0);
    assert!(!idle.step(MIN + 1));
}

#[test]
fn inhibitors_are_bounded_and_let_go_when_their_process_is_gone() {
    let mut idle = Idle::new(0);
    for pid in 1..=MAX_INHIBITORS as u32 {
        assert!(idle.inhibit(pid, true));
    }
    assert!(idle.inhibit(3, true), "again is no new place");
    assert!(!idle.inhibit(99, true), "full");
    assert!(!idle.inhibit(0, true), "no pid");
    idle.purge(|pid| pid > MAX_INHIBITORS as u32 / 2);
    idle.forget(MAX_INHIBITORS as u32);
    assert!(idle.inhibit(99, true), "room again");
    for pid in (1..MAX_INHIBITORS as u32).chain([99]) {
        idle.forget(pid);
    }
    assert!(!idle.inhibited());
}

#[test]
fn a_router_forgetting_an_exited_process_lets_go_of_its_inhibit() {
    let mut ctx = minute_desk();
    assert!(ctx.idle.inhibit(77, true));
    nonos_libc::kill(77);
    ctx.forget_pid(77);
    assert!(!ctx.idle.inhibited());
    assert!(ctx.idle.inhibit(78, true));
    nonos_libc::kill(78);
    ctx.purge_dead();
    assert!(!ctx.idle.inhibited());
}

#[test]
fn the_compositor_is_told_each_change_once_and_a_refusal_again_after_a_second() {
    let mut idle = Idle::new(0);
    assert!(idle.due(false, 0), "told nothing yet: awake is said at start");
    idle.answered(false, 0, true);
    assert!(!idle.due(false, 5), "taken: not again");
    assert!(idle.due(true, 10));
    idle.answered(true, 10, true);
    assert!(idle.due(false, 11), "a wake goes at once, a blank just went");
    idle.answered(false, 11, false);
    assert!(!idle.due(false, 11 + RETRY_MS - 1), "refused: not before a retry's time");
    assert!(idle.due(false, 11 + RETRY_MS));
    assert!(!idle.due(true, 12), "the compositor holds the blank already");
}

#[test]
fn the_wire_numbers_agree_on_both_sides() {
    use crate::router_protocol::{E_BUSY, OP_IDLE_INHIBIT};
    assert_eq!((OP_IDLE_INHIBIT, E_BUSY), (0x0007, -16), "what the Wayland bridge sends and reads");
    let theirs = include_str!("../../compositor/src/protocol/ops.rs");
    let ours = include_str!("../../capsule_input_router/src/clients/compositor/constants.rs");
    let line = "pub const OP_IDLE_BLANK: u16 = 0x001F;";
    assert!(
        theirs.contains(line) && ours.contains(line),
        "the router's IDLE_BLANK is the compositor's"
    );
    let handler = include_str!("../../compositor/src/server/handlers/idle_blank.rs");
    assert!(handler.contains("router_pid() != Some(sender)"), "only the router may blank");
}
