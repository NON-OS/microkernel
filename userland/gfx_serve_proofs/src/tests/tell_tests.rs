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

//! Saying a blank (G-89): once per silence, only after a blank ran, again
//! for a new silence after the owner was heard, never with no owner; and
//! `watch_said` hands the driver exactly BLANK_LINE.

use crate::model::wire::{ask, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use crate::watch::OWNER_SILENCE_MS;
use crate::watch_tell::BLANK_LINE;
use nonos_gfx_wire::ops;

fn owned() -> Server<Vram, Pool> {
    let mut s = Server::new(Vram::new(true, true), Pool::new(8192));
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]).0, 0);
    s.heard(COMPOSITOR, 0);
    s
}

#[test]
fn said_once_per_silence_and_only_after_the_blank() {
    let mut s = owned();
    s.watch(OWNER_SILENCE_MS - 1);
    assert!(!s.take_tell(), "nothing cleared yet");
    s.watch(OWNER_SILENCE_MS);
    assert!(s.take_tell());
    assert!(!s.take_tell(), "said once");
    s.watch(OWNER_SILENCE_MS * 3);
    assert!(!s.take_tell(), "the same silence");
}

#[test]
fn a_new_silence_after_the_owner_is_heard_is_said_again() {
    let mut s = owned();
    s.watch(OWNER_SILENCE_MS);
    assert!(s.take_tell());
    s.heard(COMPOSITOR + 1, OWNER_SILENCE_MS + 1);
    s.watch(OWNER_SILENCE_MS * 2);
    assert!(!s.take_tell(), "another pid is not the owner heard");
    s.heard(COMPOSITOR, 10_000);
    s.watch(10_000 + OWNER_SILENCE_MS);
    assert!(s.take_tell(), "a second silence");
}

#[test]
fn no_owner_is_never_said_and_the_line_is_handed_once() {
    let mut s = Server::new(Vram::new(true, true), Pool::new(8192));
    s.watch(u64::MAX);
    assert!(!s.take_tell());
    let mut s = owned();
    let mut lines = Vec::new();
    for t in [OWNER_SILENCE_MS, OWNER_SILENCE_MS + 500, OWNER_SILENCE_MS + 1000] {
        s.watch_said(t, |l| lines.push(l));
    }
    assert_eq!(lines, [BLANK_LINE]);
    assert!(BLANK_LINE.starts_with(b"[DISPLAY] ") && BLANK_LINE.ends_with(b"\n"));
}
