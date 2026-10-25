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

use super::gate_link_tests::{clock, deauth, protected};
use crate::roam::two_aps::{run_until, sent};
use crate::soft::rig::Station;
use nonos_wifi_core::pmf::SA_QUERY_MAX_MS;

fn home(s: &Station) -> bool {
    s.radio().unwrap().tuned.map(|t| t.1) == Some(1)
}

#[test]
fn a_look_between_trips_waits_at_home_while_an_sa_query_runs() {
    let mut st = protected();
    assert!(run_until(&mut st, |s| !home(s)), "the first trip");
    assert!(run_until(&mut st, home), "back home");
    let trips = sent(&st, 0x48).len();
    deauth(&mut st);
    let end = clock(&st) + SA_QUERY_MAX_MS;
    while clock(&st) < end && st.session().is_some() {
        st.tick();
        assert!(home(&st), "no trip while the query runs");
    }
    assert!(!sent(&st, 0xd0).is_empty(), "it ran the SA Query");
    assert_eq!(sent(&st, 0x48).len(), trips, "no Null frame for a trip");
}
