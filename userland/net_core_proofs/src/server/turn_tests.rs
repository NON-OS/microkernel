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


//! The serve loop runs one turn, then takes a request. With every driver
//! call it makes running to its timeout, a client's request waits at most
//! one turn, and no turn may outlast the client's 64 ms.

use super::turn::{turn, Turn, IDLE_POLL_MS, PROBE_EVERY_MS};

/// A client's budget for a reply (capsule_net_core device/budget.rs).
const CLIENT_MS: i64 = 64;
/// A poll with every driver call timing out: a batch begun just inside the
/// 8 ms ingress window (24 ms), then the four frames always sent (8 ms each).
const POLL_MS: i64 = 7 + 24 + 4 * 8;
/// The probes the same way: the bound link, then two interfaces looked for.
const PROBE_MS: i64 = 8 + 2 * 8;

fn cost(t: Turn) -> i64 {
    match t {
        Turn::Probe => PROBE_MS,
        Turn::Poll => POLL_MS,
        Turn::Neither => 0,
    }
}

#[test]
fn no_turn_outlasts_a_clients_wait_while_the_stack_is_busy() {
    let (mut now, mut probed, mut polled, mut longest) = (0i64, i64::MIN, i64::MIN, 0);
    let (mut probes, mut polls) = (0, 0);
    while now < 10 * PROBE_EVERY_MS {
        let t = turn(now, probed, polled, true);
        match t {
            Turn::Probe => (probed, probes) = (now, probes + 1),
            Turn::Poll => (polled, polls) = (now, polls + 1),
            Turn::Neither => {}
        }
        longest = longest.max(cost(t));
        now += cost(t) + 2;
    }
    assert!(longest < CLIENT_MS, "a turn ran {longest} ms");
    assert!((10..=11).contains(&probes), "{probes} probe turns in 10 s");
    assert!(polls > 100, "the probes starved the poll: {polls}");
}

#[test]
fn the_probes_take_their_turn_even_under_load_and_the_poll_the_next() {
    assert_eq!(turn(5_000, 4_000, 4_999, true), Turn::Probe);
    assert_eq!(turn(5_000, 5_000, 4_999, true), Turn::Poll);
    assert_eq!(turn(0, i64::MIN, i64::MIN, false), Turn::Probe);
}

#[test]
fn a_quiet_stack_polls_at_its_floor_and_otherwise_waits() {
    assert_eq!(turn(500, 0, 500 - IDLE_POLL_MS + 1, false), Turn::Neither);
    assert_eq!(turn(500, 0, 500 - IDLE_POLL_MS, false), Turn::Poll);
    assert_eq!(turn(500, 0, 499, true), Turn::Poll);
}
