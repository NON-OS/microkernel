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

//! How long the compositor waits on a display driver, read from its own
//! source: a FLUSH gets FLUSH_MS, above the 327 ms a whole frame was seen to
//! take while capsules spawned, and below the stall watch's STUCK_MS, so a
//! slow flush is never called a stuck loop. Cursor moves keep PRESENT_MS.

use crate::runner::stall_rule::STUCK_MS;

const CALL: &str = include_str!("../../../compositor/src/display/call.rs");
const OPS: &str = include_str!("../../../compositor/src/display/ops.rs");

fn constant(name: &str) -> u64 {
    let line = CALL.lines().find(|l| l.contains(&format!("const {name}: u64"))).expect(name);
    let v = line.split('=').nth(1).expect("a value").trim().trim_end_matches(';');
    v.replace('_', "").parse().expect("a number")
}

fn budget_of(op: &str) -> &str {
    let at = OPS.find(&format!("ops::{op},")).expect(op);
    let line = &OPS[at..OPS[at..].find('\n').map_or(OPS.len(), |e| at + e)];
    line.rsplit(", ").next().expect("the budget").split(')').next().expect("a name")
}

#[test]
fn a_flush_waits_flush_ms_and_a_cursor_present_ms() {
    assert_eq!(budget_of("FLUSH"), "FLUSH_MS");
    assert_eq!(budget_of("CURSOR"), "PRESENT_MS");
}

#[test]
fn flush_ms_covers_a_starved_frame_and_stays_under_the_stall_watch() {
    let flush = constant("FLUSH_MS");
    assert!(flush > 327, "the worst frame seen: {flush}");
    assert!((flush as i64) < STUCK_MS, "below the stall watch: {flush}");
    assert_eq!(constant("PRESENT_MS"), 100);
}
