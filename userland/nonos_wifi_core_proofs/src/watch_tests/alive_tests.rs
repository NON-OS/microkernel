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

//! An access point that stops beaconing is noticed: after `QUIET_MS` the
//! station probes it, `PROBES` times `PROBE_EVERY_MS` apart, and then the link
//! ends. A probe response, or any frame from the access point, keeps it.

use super::frames::{mgmt, BEACON, PROBE_RESPONSE};
use super::sim::{joined, AP, STA};
use nonos_wifi_core::dot11::header::BROADCAST;
use nonos_wifi_core::station::{Loss, Tick, PROBES, PROBE_EVERY_MS, QUIET_MS};

fn is_probe(t: &Tick) -> bool {
    matches!(t, Tick::Send(f) if f[0] == 0x40 && f[4..10] == AP && f[10..16] == STA)
}

#[test]
fn an_access_point_that_stops_beaconing_is_lost() {
    let mut s = joined(false);
    for t in (0..=1_000).step_by(100) {
        assert_eq!(s.tick(t), Tick::Quiet);
        s.receive(&mgmt(BEACON, BROADCAST, &[0; 12]), false);
    }
    assert_eq!(s.tick(1_000 + QUIET_MS - 1), Tick::Quiet);
    let first = 1_000 + QUIET_MS;
    for n in 0..u64::from(PROBES) {
        assert!(is_probe(&s.tick(first + n * PROBE_EVERY_MS)), "probe {n}");
        assert_eq!(s.tick(first + n * PROBE_EVERY_MS + 1), Tick::Quiet);
    }
    let end = first + u64::from(PROBES) * PROBE_EVERY_MS;
    assert_eq!(s.tick(end), Tick::Lost(Loss::Silent));
    assert!(!s.is_associated());
    assert_eq!(s.tick(end + 10_000), Tick::Quiet, "a lost link asks nothing more");
}

#[test]
fn an_answered_probe_keeps_the_link() {
    let mut s = joined(false);
    s.tick(0);
    assert!(is_probe(&s.tick(QUIET_MS)));
    s.receive(&mgmt(PROBE_RESPONSE, STA, &[0; 12]), false);
    assert_eq!(s.tick(QUIET_MS + PROBES as u64 * PROBE_EVERY_MS), Tick::Quiet);
    assert!(is_probe(&s.tick(2 * QUIET_MS)), "quiet again, asked again");
    assert!(s.is_associated());
}

#[test]
fn missed_beacons_ask_at_once_and_time_never_runs_backwards() {
    let mut s = joined(false);
    s.tick(50_000);
    s.receive(&mgmt(BEACON, BROADCAST, &[0; 12]), false);
    assert_eq!(s.tick(0), Tick::Quiet, "an earlier time is the latest one");
    assert!(is_probe(&s.tick(50_000 + QUIET_MS)));
    let mut s = joined(false);
    s.tick(10);
    s.beacons_missed();
    assert!(is_probe(&s.tick(11)), "the firmware's notice starts the asking");
}
