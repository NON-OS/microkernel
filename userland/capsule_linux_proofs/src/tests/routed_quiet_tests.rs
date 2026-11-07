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

//! A routed stream whose last ask brought nothing cost the serve loop the
//! slice net.socks5 held the poll for. It asks again only after its gap,
//! and a pass of the serve loop waits on one such stream at most, so N
//! quiet streams no longer cost N waits each pass. A stream bringing bytes
//! is never held back, and none waits past QUIET_MAX_MS.

#[path = "../../../capsule_linux/src/linux/net/routed/pass.rs"]
mod pass;
#[path = "../../../capsule_linux/src/linux/net/routed/quiet.rs"]
mod quiet;

use quiet::{Quiet, QUIET_GAP_MS, QUIET_MAX_MS};

fn free() -> bool {
    true
}

fn spent() -> bool {
    false
}

#[test]
fn a_new_or_busy_stream_asks_at_once_without_the_slot() {
    let mut q = Quiet::default();
    assert!(q.may_ask(0, || unreachable!("a new stream takes no slot")));
    q.heard(0, 512);
    assert!(q.may_ask(1, || unreachable!("a busy stream takes no slot")));
}

#[test]
fn a_quiet_stream_waits_its_gap() {
    let mut q = Quiet::default();
    q.heard(1_000, 0);
    assert!(!q.may_ask(1_000, free));
    assert!(!q.may_ask(1_000 + QUIET_GAP_MS - 1, free));
    assert!(q.may_ask(1_000 + QUIET_GAP_MS, free));
}

#[test]
fn a_quiet_stream_past_its_gap_needs_the_pass_slot_until_it_is_overdue() {
    let mut q = Quiet::default();
    q.heard(0, 0);
    assert!(!q.may_ask(QUIET_GAP_MS, spent));
    assert!(!q.may_ask(QUIET_MAX_MS - 1, spent));
    assert!(q.may_ask(QUIET_MAX_MS, spent));
}

#[test]
fn bytes_after_quiet_lift_the_gap() {
    let mut q = Quiet::default();
    q.heard(0, 0);
    q.heard(QUIET_GAP_MS, 10);
    assert!(q.may_ask(QUIET_GAP_MS, spent));
}

#[test]
fn a_clock_that_steps_back_does_not_ask_early() {
    let mut q = Quiet::default();
    q.heard(i64::MAX, 0);
    assert!(!q.may_ask(i64::MIN, free));
}

// One test holds the pass, since it is the process's one.
#[test]
fn a_pass_lends_one_quiet_wait_and_none_once_input_went() {
    let streams: Vec<Quiet> = (0..4)
        .map(|_| {
            let mut q = Quiet::default();
            q.heard(0, 0);
            q
        })
        .collect();
    pass::begin();
    let asked = streams.iter().filter(|q| q.may_ask(QUIET_GAP_MS, pass::take)).count();
    assert_eq!(asked, 1, "one quiet wait in a pass, not one per stream");
    pass::begin();
    assert!(pass::take());
    pass::begin();
    pass::spend();
    assert!(!streams[0].may_ask(QUIET_GAP_MS, pass::take), "input went: no wait");
    assert!(streams[0].may_ask(QUIET_MAX_MS, pass::take), "overdue asks anyway");
}
