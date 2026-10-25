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

//! The stall watchdog: waiting is never a stall; a turn held in one step
//! for STUCK_MS is said once with its step (and, for a request, its op and
//! sender), said again once when it goes on, and the boot's lines are
//! capped at STALL_LINES.

use crate::runner::stall_line::line;
use crate::runner::stall_rule::{Say, StallRule, STALL_LINES, STUCK_MS};
use crate::runner::stall_step::{Seen, Step};

fn seen(turn: u32, step: Step, since: i64) -> Seen {
    Seen { turn, step, since, op: 0x0002, from: 9 }
}

fn text(say: Say) -> String {
    let (buf, len) = line(say);
    String::from_utf8(buf[..len].to_vec()).expect("ascii")
}

#[test]
fn waiting_is_never_a_stall() {
    let mut r = StallRule::default();
    assert_eq!(r.check(seen(1, Step::Wait, 0), 60_000), None);
}

#[test]
fn a_step_is_stuck_at_exactly_stuck_ms_and_said_once() {
    for step in [Step::Request, Step::Upkeep, Step::Compose, Step::Flush] {
        let mut r = StallRule::default();
        assert_eq!(r.check(seen(5, step, 1000), 1000 + STUCK_MS - 1), None);
        let s = seen(5, step, 1000);
        assert_eq!(r.check(s, 1000 + STUCK_MS), Some(Say::Stuck(s, STUCK_MS)));
        assert_eq!(r.check(s, 1000 + 10 * STUCK_MS), None, "{step:?} said twice");
    }
}

#[test]
fn going_on_is_said_with_how_long_after() {
    let mut r = StallRule::default();
    let _ = r.check(seen(5, Step::Flush, 0), 2500);
    assert_eq!(r.check(seen(5, Step::Compose, 2600), 3000), Some(Say::WentOn(5, 500)));
    let mut r = StallRule::default();
    let _ = r.check(seen(5, Step::Upkeep, 0), 2500);
    assert_eq!(r.check(seen(6, Step::Upkeep, 2700), 2900), Some(Say::WentOn(5, 400)));
}

#[test]
fn the_lines_are_capped() {
    let mut r = StallRule::default();
    let mut said = 0;
    for t in 0..40u32 {
        let at = t as i64 * 10_000;
        said += r.check(seen(t, Step::Compose, at), at + STUCK_MS).is_some() as u32;
        said += r.check(seen(t, Step::Wait, at + STUCK_MS), at + STUCK_MS + 1).is_some() as u32;
    }
    assert_eq!(said, STALL_LINES as u32);
}

#[test]
fn the_lines_name_the_step_and_a_request() {
    let req = seen(7, Step::Request, 0);
    assert_eq!(
        text(Say::Stuck(req, 2000)),
        "[DISPLAY] turn 7 stuck 2000 ms in request op 0x0002 from pid 9\n"
    );
    assert_eq!(
        text(Say::Stuck(seen(7, Step::Flush, 0), 2100)),
        "[DISPLAY] turn 7 stuck 2100 ms in flush\n"
    );
    assert_eq!(text(Say::WentOn(7, 300)), "[DISPLAY] turn 7 went on after 300 ms more\n");
}
