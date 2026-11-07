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

//! Auto-lock (state/idle/auto_lock.rs): the shell is told once the
//! auto-lock time passes with no input, once a stretch; input starts the
//! count again; an idle inhibit holds it back only when Settings says so;
//! nothing is sent while locked. The notice is one the shell reads as the
//! auto-lock and never as the unproven-device notice.

use crate::route::notice::tell_idle_lock;
use crate::router_desk::*;
use crate::shell_router_notice::{is_idle_lock, is_notice};
use crate::state::idle::AutoLock;

const MIN: i64 = 60_000;

fn five_minutes(inhibit_holds: bool) -> AutoLock {
    let mut a = AutoLock::new();
    a.set(5, inhibit_holds);
    a
}

#[test]
fn zero_never_locks() {
    let a = AutoLock::new();
    assert!(!a.due(0, 1_000 * MIN, false, false));
    let mut b = five_minutes(false);
    b.set(0, false);
    assert!(!b.due(0, 1_000 * MIN, false, false));
}

#[test]
fn due_at_the_time_and_once_a_stretch() {
    let mut a = five_minutes(false);
    assert!(!a.due(1_000, 1_000 + 5 * MIN - 1, false, false));
    assert!(a.due(1_000, 1_000 + 5 * MIN, false, false));
    a.told(1_000);
    assert!(!a.due(1_000, 1_000 + 50 * MIN, false, false), "told once for the stretch");
    // New input starts a stretch of its own.
    assert!(!a.due(9_000_000, 9_000_000 + MIN, false, false));
    assert!(a.due(9_000_000, 9_000_000 + 5 * MIN, false, false));
}

#[test]
fn an_inhibit_holds_it_back_only_when_chosen() {
    let blank_only = five_minutes(false);
    assert!(blank_only.due(0, 5 * MIN, true, false), "a video does not hold back the lock");
    let both = five_minutes(true);
    assert!(!both.due(0, 5 * MIN, true, false));
    assert!(both.due(0, 5 * MIN, false, false));
}

#[test]
fn nothing_is_sent_while_locked() {
    assert!(!five_minutes(false).due(0, 50 * MIN, false, true));
}

#[test]
fn routed_input_starts_the_count_again() {
    let mut ctx = desk(800, 600);
    let a = five_minutes(false);
    let before = ctx.idle.last_input_ms;
    nonos_libc::advance_ms(4 * MIN);
    key_down(&mut ctx, b'x' as u32, 0);
    let after = ctx.idle.last_input_ms;
    assert!(after > before);
    let now = nonos_libc::mk_uptime_ms();
    assert!(!a.due(after, now + MIN, false, false));
    assert!(a.due(after, after + 5 * MIN, false, false));
}

#[test]
fn the_shell_reads_the_auto_lock_notice_as_that_alone() {
    let _ = desk(800, 600);
    assert!(tell_idle_lock(SHELL));
    let frames = nonos_libc::take_sent();
    assert_eq!(frames.len(), 1);
    let (pid, f) = &frames[0];
    assert_eq!(*pid, SHELL);
    assert!(is_idle_lock(f));
    assert!(!is_notice(f), "not the unproven-device notice");
    assert!(!tell_idle_lock(0), "no shell, nothing sent");
}

#[test]
fn the_router_reads_the_policy_and_tells_the_shell_each_pass() {
    let runner = include_str!("../../capsule_input_router/src/server/runner.rs");
    assert!(runner.contains("read_policy(&mut ctx, n > 0);"));
    assert!(runner.contains("Step::AutoLock => read_auto_lock(ctx),"));
    assert!(runner.contains("tell_auto_lock(&mut ctx);"));
    assert!(runner.contains("ctx.auto_lock.due(last, now, inhibited, ctx.session_locked)"));
    let policy = include_str!("../../capsule_input_router/src/clients/policy.rs");
    assert!(policy.contains("const FIELD_AUTO_LOCK_TIMEOUT: u32 = 0x0113;"));
    assert!(policy.contains("const FIELD_LOCK_INHIBIT_HOLDS: u32 = 0x0131;"));
    let frame = include_str!("../../capsule_desktop_shell/src/server/input/frame.rs");
    assert!(frame.contains("crate::server::lock::lock_idle(ctx);"));
}
