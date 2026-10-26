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

//! Probes back off, and the registry is asked sparingly.

use super::settle::fresh;
use crate::launch::timing::{DEADLINE_MS, LOOK_GAP_MS, PROBE_GAP_MAX_MS};
use crate::launch::Step;
use crate::state::Uptime;

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
                l.probed(Uptime(now), None);
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
