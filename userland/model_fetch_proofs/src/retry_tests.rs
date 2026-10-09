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

/*
 * When a failed or dropped download is tried again, and when it is given
 * up and as what: through Nym or Anyone, six tries in a row at most and
 * three minutes at most, waits from 5 s to 30 s, any byte starting the
 * count again; directly, three rounds over the mirrors. Given up: no
 * session and nothing opened is unreachable; any other anonymity failure
 * is no exit answering; directly, nothing reached is unreachable and
 * anything else is no mirror serving it.
 */

use crate::retry::{End, Kind, Next, Retry, PATIENCE_MS, ROTATIONS};

fn run(anonymous: bool, kind: Kind, opened: bool, got_any: bool) -> (Vec<i64>, End) {
    let mut r = Retry::new(0);
    r.opened = opened;
    let (mut waits, mut now) = (Vec::new(), 0);
    loop {
        match r.after(anonymous, 2, false, got_any, kind, now) {
            Next::Wait { ms, n, of } => {
                assert_eq!(n as usize, waits.len() + 2);
                assert_eq!(of, if anonymous { ROTATIONS } else { 6 });
                waits.push(ms);
                now += ms;
            }
            Next::GiveUp(end) => return (waits, end),
        }
    }
}

#[test]
fn an_anonymity_network_gets_six_tries_backing_off_to_thirty_seconds() {
    let (waits, end) = run(true, Kind::Other, false, false);
    assert_eq!(waits, [5_000, 10_000, 20_000, 30_000, 30_000]);
    assert!(waits.iter().sum::<i64>() < PATIENCE_MS);
    assert_eq!(end, End::NoExit);
}

#[test]
fn no_session_and_nothing_opened_is_unreachable_not_a_silent_exit() {
    assert_eq!(run(true, Kind::NoSession, false, false).1, End::Unreachable);
    /* A connection that opened, or bytes that came, make it the exits'. */
    assert_eq!(run(true, Kind::NoSession, true, false).1, End::NoExit);
    assert_eq!(run(true, Kind::NoSession, false, true).1, End::NoExit);
}

#[test]
fn a_mix_of_no_session_and_silent_exits_is_no_exit() {
    let mut r = Retry::new(0);
    assert!(matches!(r.after(true, 1, false, false, Kind::NoSession, 0), Next::Wait { .. }));
    let mut end = None;
    for i in 0..ROTATIONS {
        if let Next::GiveUp(e) = r.after(true, 1, false, false, Kind::Other, i64::from(i)) {
            end = Some(e);
            break;
        }
    }
    assert_eq!(end, Some(End::NoExit));
}

#[test]
fn bytes_that_came_start_the_count_again() {
    let mut r = Retry::new(0);
    for _ in 0..ROTATIONS - 1 {
        assert!(matches!(r.after(true, 1, false, true, Kind::Other, 0), Next::Wait { .. }));
    }
    assert_eq!(r.after(true, 1, true, true, Kind::Other, 1), Next::Wait { ms: 5_000, n: 2, of: ROTATIONS });
}

#[test]
fn three_minutes_is_the_most_a_run_of_tries_takes() {
    let mut r = Retry::new(0);
    assert!(matches!(r.after(true, 1, false, false, Kind::Other, 0), Next::Wait { .. }));
    assert_eq!(r.after(true, 1, false, false, Kind::Other, PATIENCE_MS), Next::GiveUp(End::NoExit));
}

#[test]
fn a_direct_connection_keeps_its_rounds_and_its_ends() {
    let (waits, end) = run(false, Kind::Other, false, false);
    assert_eq!(waits, [1_000, 2_000, 3_000, 4_000, 5_000]);
    assert_eq!(end, End::Unreachable);
    assert_eq!(run(false, Kind::Other, true, false).1, End::NoMirror);
}
