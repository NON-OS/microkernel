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

//! A boxed run is read to its report, and the report waits for the shell's
//! screen, said to be cut short if it was. The glue calls libc and is not
//! mounted here; held to the order it must keep by its text.

#[test]
fn a_boxed_run_is_read_to_its_report_and_drawn_when_the_screen_is_the_shells() {
    let ext = include_str!("../../../capsule_terminal/src/jobs/external.rs");
    let waiting = ext.find("box_wait::waiting(").unwrap();
    assert!(waiting < ext.find("stdin.feed(pid)").unwrap());
    assert!(ext.contains("super::box_wait::ended(boxed, status, out, leave_modes)"));
    let io = include_str!("../../../capsule_terminal/src/jobs/external_io.rs");
    assert!(io.contains("let all = capture.is_some() || boxed.is_some();"));
    assert!(io.contains("hold.take(&buf[..(n as usize).min(buf.len())]);"));
    let draw = include_str!("../../../capsule_terminal/src/jobs/box_draw.rs");
    let cut = "None if !hold.lines().0.is_empty() => missing(Missing::Incomplete, exit, out),";
    assert!(draw.contains(cut), "a report cut short is said to be");
    let none = "if shown.is_empty() {\n        missing(Missing::None, exit, out);";
    assert!(draw.contains(none), "a report that never came is said not to have");
    let wait = include_str!("../../../capsule_terminal/src/jobs/box_wait.rs");
    let kept = wait.find("if held {\n        return Some(JobProgress::Running);").unwrap();
    assert!(kept < wait.find("super::box_draw::draw(hold, out);").unwrap());
    assert!(wait.contains("let status = hold.ended?;"));
    let noted = wait.find("hold.ended = Some(status);").unwrap();
    assert!(noted < wait.find("waiting(Some(hold), out, held)").unwrap());
}
