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

//! One session at a time, ended only by its owner, and locked again when its
//! owner ends without ending it.

use crate::protocol::{E_AUTH, E_BUSY};
use crate::state::Context;

const OWNER: u32 = 40;
const OTHER: u32 = 41;
const KEY: u32 = 7;

fn context() -> Context {
    Context::new(1, 2, 3, 640, 480, 2560, 0)
}

/// The session's state word: 0 locked, 1 unlocked.
fn unlocked(ctx: &Context) -> bool {
    ctx.state_words().0 == 1
}

#[test]
fn only_the_owner_ends_a_session_and_a_second_waits() {
    let mut ctx = context();
    assert!(ctx.start_session(OWNER, KEY).is_ok());
    assert_eq!(ctx.start_session(OTHER, KEY), Err(E_BUSY), "one session at a time");
    assert_eq!(ctx.end_session(OTHER), Err(E_AUTH), "another pid cannot end it");
    assert!(unlocked(&ctx));
    assert_eq!(ctx.end_session(OWNER), Ok(()));
    assert!(!unlocked(&ctx));
}

/*
 * Only the owner ends its session, so an owner that ended without ending it
 * left the machine unlocked in its name, and every later start was refused
 * as busy until a reboot.
 */
#[test]
fn the_session_of_an_ended_owner_is_locked_and_a_new_one_starts() {
    let mut ctx = context();
    assert!(ctx.start_session(OWNER, KEY).is_ok());
    assert_eq!(ctx.end_if_owner_ended(|pid| pid != OWNER), Some(KEY));
    assert!(!unlocked(&ctx));
    assert_eq!(ctx.current_key_id(), None);
    assert!(ctx.start_session(OTHER, KEY).is_ok(), "a new session starts");
}

#[test]
fn a_living_owner_keeps_its_session_and_a_locked_machine_stays_locked() {
    let mut ctx = context();
    assert_eq!(ctx.end_if_owner_ended(|_| false), None, "nothing open, nothing to lock");
    assert!(ctx.start_session(OWNER, KEY).is_ok());
    assert_eq!(ctx.end_if_owner_ended(|_| true), None);
    assert!(unlocked(&ctx));
    assert_eq!(ctx.state_words().1, OWNER);
}
