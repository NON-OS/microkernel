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

//! What a killed caller's failed asks may and may not decide (L81). Its
//! transport failure reads as a refusal, never as an absent disk, and no disk
//! is kept for it after the boot disk's wait; and the entropy generator's lock
//! is never held across an ask of the capsule, so a caller killed in one
//! leaves no lock behind for every later caller to spin on.

use super::text::{between, code};

const FOUND: &str = include_str!("../../../../src/hardware/block_device/found.rs");
// The pass's end is settled in kept.rs, which select.rs calls.
const SELECT: &str = include_str!("../../../../src/hardware/block_device/kept.rs");
const WAIT: &str = include_str!("../../../../src/hardware/block_device/wait.rs");
const FAST: &str = include_str!("../../../../src/security/entropy_capsule/fast.rs");
const ASK: &str = include_str!("../../../../src/security/entropy_capsule/client/ask.rs");

#[test]
fn a_killed_callers_failure_never_settles_the_disk() {
    let refused = between(FOUND, "BlockDeviceError::Stale", "_ => Found::Absent,");
    assert!(refused.contains(&code("| BlockDeviceError::TransportFailure")), "read as absent");
    assert!(refused.ends_with(&code("=> Found::Refused(e),")), "a refusal no longer refused");
    let waited = "(Some((b, p)), Some(e)) => { if waited_for_boot_disk() && !caller_gone() {";
    assert!(code(SELECT).contains(&code(waited)), "a disk kept for a killed caller");
    let gone = concat!(
        "pub(super) fn caller_gone() -> bool { crate::process::current_pid()",
        ".is_some_and(|p| crate::process::scheduler::selection::is_dead(p)) }",
    );
    let c = code(WAIT);
    assert_eq!(&c[c.find(&code("pub(super) fn caller_gone(")).expect("gone")..], code(gone));
}

#[test]
fn the_generator_lock_is_let_go_across_the_reseed() {
    // The capsule is asked before STATE is taken and never waited on (K9).
    let fill = between(FAST, "pub fn fill(", "let Some(state) = guard.as_mut() else");
    let ask = fill.find(&code("super::client::poll();")).expect("the capsule asked");
    let held = fill.find(&code("let mut guard = STATE.lock();")).expect("the lock taken");
    assert!(ask < held, "the capsule asked under STATE");
    assert_eq!(fill.matches("STATE.lock()").count(), 1, "STATE taken somewhere else too");
    assert!(!fill.contains("client::get_random"), "a round trip back inside the fill");
    let ask = code(ASK);
    assert!(ask.contains(&code("transport_nowait::{landed, post}")), "the ask waits");
    assert!(!ask.contains("round_trip") && !ask.contains("reply_wait"), "the ask waits");
}
