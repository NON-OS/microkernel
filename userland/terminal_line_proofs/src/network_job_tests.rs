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

//! `http` and `git clone` run as stepped jobs. Each tick the job asks the
//! clock about the far end: a direct socket's quiet ends a response, a quiet
//! through an anonymity network is a stall and fails it, the total bound
//! ends anything, and a handshake is never ended by quiet or a close. A
//! clone's git steps leave their one request in an `Ask` and get its answer
//! back once, never an answer meant for another request.

use crate::exchange_wait::{
    settle_handshake, settle_response, Heard, Settle, Wait, ANON_QUIET_MS, ANON_TOTAL_MS,
    CLOSED_HANDSHAKE, DIRECT_QUIET_MS, DIRECT_TOTAL_MS, QUIET_ANON, QUIET_HANDSHAKE, TOTAL_ANON,
    TOTAL_DIRECT,
};
use crate::git_ask::Ask;

#[test]
fn a_direct_wait_goes_quiet_then_runs_out() {
    let w = Wait::new(0, false);
    assert_eq!(w.check(DIRECT_QUIET_MS - 1), Heard::Waiting);
    assert_eq!(w.check(DIRECT_QUIET_MS), Heard::Quiet);
    assert_eq!(w.check(DIRECT_TOTAL_MS), Heard::Total);
}

#[test]
fn bytes_restart_the_quiet_but_not_the_total() {
    let mut w = Wait::new(0, false);
    let mut now = 0;
    while now < DIRECT_TOTAL_MS - 1_000 {
        now += 1_000;
        w.heard(now);
        assert_eq!(w.check(now), Heard::Waiting, "at {now}");
    }
    assert_eq!(w.check(DIRECT_TOTAL_MS), Heard::Total);
}

#[test]
fn an_anonymous_wait_is_longer() {
    let w = Wait::new(0, true);
    assert_eq!(w.check(DIRECT_QUIET_MS), Heard::Waiting);
    assert_eq!(w.check(ANON_QUIET_MS), Heard::Quiet);
    assert_eq!(w.check(ANON_TOTAL_MS), Heard::Total);
}

#[test]
fn a_response_ends_when_the_far_end_says_so() {
    assert_eq!(settle_response(Heard::Waiting, true, true), Settle::Finish);
    assert_eq!(settle_response(Heard::Quiet, true, false), Settle::Finish);
    assert_eq!(settle_response(Heard::Waiting, false, false), Settle::More);
}

#[test]
fn quiet_ends_a_direct_response_and_fails_an_anonymous_one() {
    assert_eq!(settle_response(Heard::Quiet, false, false), Settle::Finish);
    assert_eq!(settle_response(Heard::Quiet, false, true), Settle::Fail(QUIET_ANON));
    assert_eq!(settle_response(Heard::Total, false, false), Settle::Fail(TOTAL_DIRECT));
    assert_eq!(settle_response(Heard::Total, false, true), Settle::Fail(TOTAL_ANON));
}

#[test]
fn a_handshake_is_never_ended_by_a_close_or_quiet() {
    assert_eq!(settle_handshake(Heard::Waiting, false, false), Settle::More);
    assert_eq!(settle_handshake(Heard::Waiting, true, true), Settle::Fail(CLOSED_HANDSHAKE));
    assert_eq!(settle_handshake(Heard::Quiet, false, false), Settle::Fail(QUIET_HANDSHAKE));
    assert_eq!(settle_handshake(Heard::Quiet, false, true), Settle::Fail(QUIET_ANON));
    assert_eq!(settle_handshake(Heard::Total, false, true), Settle::Fail(TOTAL_ANON));
}

#[test]
fn a_step_asks_then_gets_its_answer_once() {
    let mut ask = Ask::new();
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), None);
    let carried = ask.take_asked().expect("the request is left for the job");
    assert_eq!(carried, b"GET /info/refs".to_vec());
    assert_eq!(ask.take_asked(), None);
    ask.answered(carried, b"refs".to_vec());
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), Some(b"refs".to_vec()));
    // Taken: the same request asks again rather than reading a stale answer.
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), None);
}

#[test]
fn an_answer_is_never_handed_to_another_request() {
    let mut ask = Ask::new();
    ask.answered(b"GET /info/refs".to_vec(), b"refs".to_vec());
    assert_eq!(ask.request(b"POST /git-upload-pack".to_vec()), None);
    assert_eq!(ask.take_asked(), Some(b"POST /git-upload-pack".to_vec()));
    // The mismatched answer was dropped, not kept for later.
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), None);
}
