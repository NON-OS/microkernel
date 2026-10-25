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

//! A boxed run stopped by Ctrl+C: its report frames kept from what is
//! dropped, the report drawn last if it came whole, and kept while another
//! program holds the screen. The glue calls libc; held to its order by its
//! text.

fn at(text: &str, what: &str) -> usize {
    text.find(what).unwrap_or_else(|| panic!("missing {what:?}"))
}

#[test]
fn a_stopped_run_keeps_its_report_and_draws_it_last() {
    let step = include_str!("../../../capsule_terminal/src/jobs/work_step.rs");
    assert!(step.contains("return super::super::external_stop::stop_external("));
    let stop = include_str!("../../../capsule_terminal/src/jobs/external_stop.rs");
    let order = [
        "box_wait::waiting(boxed.as_deref_mut(), out, leave_modes)",
        "discard_output(pid, boxed.as_deref_mut());",
        "out.program_ended();",
        "capture.interrupted(out);",
        "out.writeln(b\"interrupted\");",
        "super::box_wait::stopped(boxed, out, leave_modes)",
    ];
    let places: Vec<usize> = order.iter().map(|w| at(stop, w)).collect();
    assert!(places.windows(2).all(|p| p[0] < p[1]), "{places:?}");
}

#[test]
fn only_a_whole_report_is_drawn_after_a_stop() {
    let wait = include_str!("../../../capsule_terminal/src/jobs/box_wait.rs");
    let stopped = &wait[at(wait, "pub(super) fn stopped(")..];
    let cut = at(stopped, "Some(hold) if !hold.complete() => {");
    assert!(cut < at(stopped, "box_draw::missing(super::box_missing::Missing::Stopped, STOPPED"));
    assert!(stopped.contains("hold => ended(hold, STOPPED, out, held),"));
    /* A shell's status for a program ended by SIGINT (2): 128 and the signal. */
    assert!(wait.contains(&format!("pub(super) const STOPPED: i32 = {};", 128 + 2)));
}
