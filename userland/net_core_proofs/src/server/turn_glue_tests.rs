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


//! The serve loop keeps to the turn rule: the probe turn and the poll turn
//! each run their own calls only, the poll no longer looks at the link, and
//! the loop's clock is uptime.

const RUN: &str = include_str!("../../../capsule_net_core/src/server/runner/run.rs");
const POLL: &str = include_str!("../../../capsule_net_core/src/iface/poll.rs");

fn arm(name: &str) -> &'static str {
    let at = RUN.find(&format!("Turn::{name} => {{")).expect("arm missing");
    &RUN[at..at + RUN[at..].find("            }\n").expect("arm unclosed")]
}

#[test]
fn a_probe_turn_never_polls_and_a_poll_turn_never_probes() {
    let (probe, poll) = (arm("Probe"), arm("Poll"));
    for call in ["relink::check()", "setup::reevaluate()", "autojoin::tick(now)"] {
        assert!(probe.contains(call) && !poll.contains(call), "{call}");
    }
    assert!(poll.contains("poll::pump()") && !probe.contains("pump()"));
    assert!(!POLL.contains("relink::check"), "the poll looks at the link again");
}

#[test]
fn the_loop_runs_on_uptime() {
    assert!(RUN.contains("let now = mk_uptime_ms();"));
    assert!(!RUN.contains("mk_time_millis"));
}
