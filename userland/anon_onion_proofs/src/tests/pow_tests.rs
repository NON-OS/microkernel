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


//! A service under load, against the simulated network: the client solves
//! the puzzle its descriptor asks for, sends a fresh solution with each
//! introduction at the effort hs_client.c would choose, gives up solving
//! when its time runs out, and drops the work when its caller goes.

use super::world::{net, Faults};

fn loaded(effort: u32) -> Faults {
    Faults { pow_effort: Some(effort), ..Faults::default() }
}

#[test]
fn a_service_under_load_gets_a_valid_solution_and_answers() {
    let mut n = net(loaded(8), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open", "{:#?}", n.log());
    assert_eq!(n.world.pow_seen, [Some((8, true))], "one introduction, solved at the suggested effort");
    let log = n.log();
    assert!(log.iter().any(|l| l.contains("onion service asks for a puzzle, effort")));
    assert!(log.iter().any(|l| l.contains("onion puzzle solved, equi-x instances")));
}

#[test]
fn a_service_not_under_load_gets_no_extension() {
    let mut n = net(Faults::default(), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    assert_eq!(n.world.pow_seen, [None]);
}

#[test]
fn a_suggested_effort_of_zero_is_not_solved() {
    let mut n = net(loaded(0), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    assert_eq!(n.world.pow_seen, [None]);
}

#[test]
fn each_refusal_raises_the_effort_and_every_nonce_is_fresh() {
    // Two of the three introduction points refuse; whichever order the
    // client tries them in, each introduction after a refusal is solved at
    // the next effort hs_client.c names, and none reuses a nonce (the
    // service's replay check would fail it).
    let mut n = net(loaded(8), |relays| {
        for r in relays.iter_mut() {
            if matches!(r.intro_index, Some(0) | Some(1)) {
                r.refuse = Some(2);
            }
        }
    });
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "open", "{:#?}", n.log());
    let seen = &n.world.pow_seen;
    assert!(seen.len() >= 2, "at least one refusal before the good point: {seen:?}");
    let expected = [8, 16, 32];
    for (i, s) in seen.iter().enumerate() {
        assert_eq!(*s, Some((expected[i], true)), "introduction {i}");
    }
}

#[test]
fn solving_stops_at_its_time_and_the_introduction_goes_without() {
    // At the client's cap a solve takes thousands of Equi-X instances. Each
    // idle turn does a bounded slice; with ten seconds between slices the
    // ninety second solving window closes long before a solution, and the
    // introduction goes out without one, which the service still accepts.
    let mut n = net(loaded(10_000), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(12);
    assert!(n.log().iter().any(|l| l.contains("onion service asks for a puzzle")), "solving has begun");
    for _ in 0..30 {
        n.pass(10);
        n.run(3);
        if n.stream(id).0 != "opening" {
            break;
        }
    }
    let log = n.log();
    assert_eq!(n.stream(id).0, "open", "{log:#?}");
    assert_eq!(n.world.pow_seen, [None], "introduced without a solution");
    assert!(log.iter().any(|l| l.contains("onion puzzle not solved in time, introducing without a solution")));
    assert!(!log.iter().any(|l| l.contains("onion puzzle solved")));
}

#[test]
fn a_caller_that_goes_mid_solve_takes_the_puzzle_with_it() {
    let mut n = net(loaded(10_000), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(12);
    assert!(n.log().iter().any(|l| l.contains("onion service asks for a puzzle")), "solving has begun");
    assert_eq!(n.state.onion.len(), 1);
    n.state.streams.retain(|s| s.id != id);
    n.run(1);
    assert!(n.state.onion.is_empty(), "the lookup, and its solver, are gone");
    assert!(n.world.pow_seen.is_empty(), "nothing was ever introduced");
}

#[test]
fn the_lookup_deadline_makes_room_for_the_solving_window() {
    // A lookup normally gives up after LOOKUP_SECONDS; one that is solving
    // has its deadline moved out once to cover the window and an
    // introduction after it, and no further.
    let mut n = net(loaded(10_000), |_| {});
    let started = n.now;
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(12);
    let deadline = n.state.onion[0].deadline;
    assert!(deadline > started + 45, "moved past the plain lookup deadline");
    assert!(deadline <= started + 45 + 90 + 30, "by the window and two steps at most");
    assert_eq!(n.stream(id).0, "opening");
}
