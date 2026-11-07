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
//! A second press on the title bar soon after the first and close to it is
//! a double click, and the window is maximized, or restored when it is, as
//! its maximize control does. Pure, so the host proofs hold it
//! (desktop_proofs drag_tests double).

/// The longest time between the two presses.
pub(in crate::runner) const DOUBLE_MS: i64 = 400;
/// How far apart the two presses may be, in pixels each way.
pub(in crate::runner) const DOUBLE_PX: u32 = 4;

const NS_PER_MS: u64 = 1_000_000;

/// When a press came, in uptime milliseconds: the time its event carries
/// (the input router's, from the moment it took the event from the kernel),
/// else `now_ms`, when the runner read it. A runner busy drawing reads the
/// second press of a double click late, and timed by that read the two
/// came more than DOUBLE_MS apart.
pub(in crate::runner) fn press_ms(timestamp_ns: u64, now_ms: i64) -> i64 {
    if timestamp_ns == 0 {
        return now_ms;
    }
    i64::try_from(timestamp_ns / NS_PER_MS).unwrap_or(now_ms)
}

/// Whether the pointer, `travel` from where the title bar was pressed, has
/// gone further than a double click's two presses may be apart, so the
/// press is a drag. A tablet says where the pointer is with every button
/// report, so a click comes with motion that goes nowhere; that motion
/// leaves the press half of a double click.
pub(in crate::runner) fn moved_off(travel: (i64, i64)) -> bool {
    let reach = u64::from(DOUBLE_PX);
    travel.0.unsigned_abs() > reach || travel.1.unsigned_abs() > reach
}

/// The first press of what may be a double click: when, and where in the
/// window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::runner) struct TitlePress {
    pub at_ms: i64,
    pub x: i32,
    pub y: i32,
}

/// A press on the title bar at `x`, `y`, at `now_ms` when the runner's clock
/// is known: whether it is the second of a double click. One that is not is
/// kept as the first of the next; one that is clears it, so a third press
/// starts again. A press with no clock known is never the second, and is not
/// kept.
pub(in crate::runner) fn second_press(
    last: &mut Option<TitlePress>,
    now_ms: Option<i64>,
    (x, y): (i32, i32),
) -> bool {
    let Some(now) = now_ms else {
        *last = None;
        return false;
    };
    let soon =
        |p: &TitlePress| now.checked_sub(p.at_ms).is_some_and(|g| (0..=DOUBLE_MS).contains(&g));
    let near = |p: &TitlePress| x.abs_diff(p.x) <= DOUBLE_PX && y.abs_diff(p.y) <= DOUBLE_PX;
    if last.is_some_and(|p| soon(&p) && near(&p)) {
        *last = None;
        return true;
    }
    *last = Some(TitlePress { at_ms: now, x, y });
    false
}
