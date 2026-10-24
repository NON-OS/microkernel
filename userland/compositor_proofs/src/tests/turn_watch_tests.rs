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

//! The serve loop's lines: serving, the first request and the first frame
//! each said once, and a turn of SLOW_MS or more said with where its time
//! went, at most SLOW_LINES times, so a boot that stalls the loop says how
//! without filling the log.

use crate::turn_watch::{Turn, TurnWatch, SLOW_LINES, SLOW_MS};

fn turn(at: [i64; 4], request: Option<(u16, u32)>, released: bool) -> Turn {
    Turn { at, request, released }
}

#[test]
fn serving_the_first_request_and_the_first_frame_are_each_said_once() {
    let mut w = TurnWatch::default();
    let first = w.observe(&turn([100, 116, 117, 118], None, false));
    assert_eq!(first, ["serving: first turn at 100 ms took 18 ms"]);
    let asked = w.observe(&turn([200, 210, 211, 212], Some((0x0001, 9)), false));
    assert_eq!(asked, ["first request op 0x0001 from pid 9 at 210 ms"]);
    assert!(w.observe(&turn([300, 310, 311, 312], Some((0x0008, 9)), false)).is_empty());
    assert_eq!(
        w.observe(&turn([400, 401, 402, 410], None, true)),
        ["first frame let through at 410 ms"]
    );
    assert!(w.observe(&turn([500, 501, 502, 503], Some((0x0002, 9)), true)).is_empty());
}

#[test]
fn a_slow_turn_says_where_its_time_went_at_most_slow_lines_times() {
    let mut w = TurnWatch::default();
    let _ = w.observe(&turn([0, 1, 2, 3], None, false));
    let slow = |t0: i64| turn([t0, t0 + 2, t0 + 3, t0 + 3 + SLOW_MS], None, false);
    let said = w.observe(&slow(1_000));
    assert_eq!(
        said,
        [format!("slow turn {} ms (drain 2, upkeep 1, frame {})", SLOW_MS + 3, SLOW_MS)]
    );
    for k in 1..SLOW_LINES as i64 {
        assert_eq!(w.observe(&slow(10_000 * k)).len(), 1);
    }
    assert!(w.observe(&slow(99_000)).is_empty(), "past SLOW_LINES the log stays quiet");
    let just_under = turn([200_000, 200_001, 200_002, 200_000 + SLOW_MS - 1], None, false);
    assert!(TurnWatch::default().observe(&just_under).iter().all(|l| !l.starts_with("slow")));
    let exactly = turn([0, 0, 0, SLOW_MS], None, false);
    assert!(TurnWatch::default().observe(&exactly).iter().any(|l| l.starts_with("slow")));
}
