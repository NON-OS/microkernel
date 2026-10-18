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

//! How often the jack is read, against a codec that answers and one that
//! has stopped. Time here is the schedule's own: milliseconds handed in.

use crate::server::runner::jack_schedule::{JackSchedule, JackState, MAX_FAILS, POLL_MS};

/// The times the schedule reads the jack over `ms`, polled every 10 ms as
/// the serving loop does, with each read answered or not by `answers`.
fn reads(s: &mut JackSchedule, from: u64, ms: u64, answers: bool) -> Vec<u64> {
    let mut at = Vec::new();
    for now in (from..from + ms).step_by(10) {
        if s.due(now, 0) {
            s.answered(answers);
            at.push(now);
        }
    }
    at
}

#[test]
fn a_codec_that_answers_is_read_twice_a_second() {
    let mut s = JackSchedule::new(0);
    // The first read is half a second in; nineteen more fill ten seconds.
    assert_eq!(reads(&mut s, 0, 10_000, true).len() as u64, 10_000 / POLL_MS - 1);
    assert_eq!((s.interval(), s.state()), (POLL_MS, JackState::Answering));
}

#[test]
fn a_codec_that_stops_answering_is_read_less_often_then_not_at_all() {
    let mut s = JackSchedule::new(0);
    let at = reads(&mut s, 0, 600_000, false);
    assert_eq!(at.len() as u32, MAX_FAILS, "read at {at:?} in ten minutes");
    let gaps: Vec<u64> = at.windows(2).map(|w| w[1] - w[0]).collect();
    assert_eq!(gaps, [1_000, 2_000, 4_000, 8_000, 16_000], "the waits do not double");
    assert_eq!(s.state(), JackState::Lost);
    assert!(!s.due(u64::MAX, 7), "a lost codec was asked again on a jack event");
}

#[test]
fn an_answer_before_the_codec_is_lost_brings_the_half_second_back() {
    let mut s = JackSchedule::new(0);
    reads(&mut s, 0, 7_000, false);
    assert_eq!(s.state(), JackState::Retrying(3));
    s.answered(true);
    assert_eq!((s.interval(), s.state()), (POLL_MS, JackState::Answering));
}

#[test]
fn an_unsolicited_response_reads_the_jack_at_once_while_retrying() {
    let mut s = JackSchedule::new(0);
    let quiet = reads(&mut s, 0, 7_000, false).len();
    assert!(!s.due(7_010, 0), "read before its interval with nothing new");
    assert!(s.due(7_010, 1), "a jack event waited out the back-off");
    assert!(quiet > 0 && !s.due(7_020, 1), "the same event read the jack twice");
}
