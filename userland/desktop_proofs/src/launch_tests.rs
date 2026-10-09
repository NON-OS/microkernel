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

//! A Launchpad launch the installer is still loading is followed on the
//! shell's turns: the app showing up starts it, an installer that answers a
//! probe with no app fails it, and the deadline ends it either way. Probes
//! back off so the shell never holds more than the kernel's eight places in
//! the installer's reply queue.

use crate::launch::{Launch, Step, DEADLINE_MS, FIRST_PROBE_MS, LOOK_GAP_MS, PROBE_GAP_MAX_MS};
use crate::state::Uptime;

fn fresh() -> Launch {
    Launch::new(b"notes", vec![3, 4], Uptime(1_000))
}

#[test]
fn a_new_launch_waits_before_its_first_probe() {
    let l = fresh();
    assert_eq!(l.step(Uptime(1_000), None), Step::Wait);
    assert_eq!(l.step(Uptime(1_000 + FIRST_PROBE_MS - 1), None), Step::Wait);
    assert_eq!(l.step(Uptime(1_000 + FIRST_PROBE_MS), None), Step::Probe);
}

#[test]
fn the_app_appearing_starts_it_at_any_point() {
    let mut l = fresh();
    assert_eq!(l.step(Uptime(1_010), Some(42)), Step::Started(42));
    l.probed(Uptime(1_600), true);
    assert_eq!(l.step(Uptime(1_600), Some(42)), Step::Started(42));
    assert_eq!(l.step(Uptime(1_000 + DEADLINE_MS + 5), Some(42)), Step::Started(42));
}

#[test]
fn an_answered_probe_with_no_app_fails_it() {
    let mut l = fresh();
    l.probed(Uptime(1_500), true);
    assert!(l.served());
    assert_eq!(l.step(Uptime(1_500), None), Step::Failed);
}

#[test]
fn the_deadline_fails_a_launch_that_never_settles() {
    let l = fresh();
    assert_ne!(l.step(Uptime(1_000 + DEADLINE_MS - 1), None), Step::Failed);
    assert_eq!(l.step(Uptime(1_000 + DEADLINE_MS), None), Step::Failed);
}

#[test]
fn unanswered_probes_back_off_and_hold_few_places() {
    let mut l = fresh();
    let mut now = 1_000;
    let mut probes = 0;
    let mut last_gap = 0;
    let mut last_probe = now;
    while now < 1_000 + DEADLINE_MS {
        match l.step(Uptime(now), None) {
            Step::Probe => {
                probes += 1;
                let gap = now - last_probe;
                assert!(gap >= last_gap.min(PROBE_GAP_MAX_MS));
                last_gap = gap;
                last_probe = now;
                l.probed(Uptime(now), false);
            }
            Step::Wait => {}
            other => panic!("unexpected {other:?} at {now}"),
        }
        now += 10;
    }
    // The load's own place and these together stay under the kernel's eight.
    assert!(probes < 8, "{probes} probes");
    assert!(probes >= 3, "{probes} probes");
}

#[test]
fn the_registry_is_asked_at_most_every_look_gap() {
    let mut l = fresh();
    assert!(l.due(Uptime(1_000)));
    assert!(!l.due(Uptime(1_000 + LOOK_GAP_MS - 1)));
    assert!(l.due(Uptime(1_000 + LOOK_GAP_MS)));
}

#[test]
fn it_keeps_the_children_seen_before_the_load() {
    let l = fresh();
    assert_eq!(l.children, vec![3, 4]);
    assert_eq!(l.name, b"notes".to_vec());
    assert_eq!(l.elapsed_ms(Uptime(1_250)), 250);
}

/// Started in the shell's first seconds, when the wall clock still read its
/// error (-61) and never moved: on uptime the launch is due again a look gap
/// later and fails at its deadline, rather than waiting forever and holding
/// every later Launchpad launch behind it.
#[test]
fn a_launch_started_early_is_followed_to_its_end() {
    let mut l = Launch::new(b"notes", vec![], Uptime(40));
    assert!(l.due(Uptime(40)));
    assert!(l.due(Uptime(40 + LOOK_GAP_MS)));
    assert_eq!(l.step(Uptime(40 + DEADLINE_MS), None), Step::Failed);
}
