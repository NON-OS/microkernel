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

//! A closed connection the stack still holds is let go within Linux's FIN
//! timeout, and never more than MOST are held, however many connections
//! come and go: net.core's memory for them stays flat (state/orphan_rule.rs).

#[path = "../../../capsule_net_core/src/state/orphan_rule.rs"]
mod orphan_rule;

use orphan_rule::{goes, LINGER_MS, MOST};

/// One reap over orphans adopted at `ages` (oldest first) and `done`.
fn reap(held: &mut Vec<(i64, bool)>, now: i64) {
    let count = held.len();
    let mut index = 0;
    held.retain(|&(at, done)| {
        let gone = goes(done, now - at, index, count);
        index += 1;
        !gone
    });
}

#[test]
fn connections_whose_peer_never_closes_stay_bounded_however_many_come() {
    // Each second a GET on a fresh connection to a keep-alive server, closed
    // by its client, which the server never closes: FIN-WAIT-2 for good.
    let mut held = Vec::new();
    for second in 0..10_000i64 {
        held.push((second * 1000, false));
        reap(&mut held, second * 1000);
        assert!(held.len() <= MOST, "at {second} s, {} held", held.len());
    }
    assert_eq!(held.len(), MOST, "the newest MOST, each younger than LINGER_MS");
}

#[test]
fn one_over_or_out_of_time_goes_and_a_young_one_stays() {
    assert!(goes(true, 0, 5, 6), "its exchange is over");
    assert!(goes(false, LINGER_MS, 5, 6), "it waited as long as Linux would");
    assert!(!goes(false, LINGER_MS - 1, 5, 6), "a young one finishes with its peer");
    assert!(goes(false, 0, 0, MOST + 1), "the oldest past MOST");
    assert!(!goes(false, 0, 1, MOST + 1), "the next is within MOST");
}

#[test]
fn the_stack_resets_one_it_lets_go_before_its_exchange_ends() {
    let src = include_str!("../../../capsule_net_core/src/state/orphans.rs");
    let at = |s: &str| src.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("let gone = goes(done, now.wrapping_sub(at), index, count);") < at("sock.abort();"));
    assert!(at("sock.abort();") < at("sockets.remove(h);"));
}
