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

//! Nothing reset, so nothing is let go: the driver's own stall in a slow
//! set-up while a stick moves and the finger keeps its click, on a pad that
//! reads its mode back and on one that does not.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_libc::{mk_idle_ms, mk_uptime_ms, take_events, INPUT_KIND_BUTTON_UP};

use super::behind::dragging;
use crate::input::poll;
use crate::tests::fixture::of_kind;

/// Three seconds of a stick on the mouse collection, three of its reports
/// to each touch report, the click held, while each set-up's GET (a pad
/// that reads its mode back) or SET (one that does not) takes `slow` ms.
fn stall(slow: u64, read_back: bool) -> usize {
    let (f, mut s) = dragging();
    let slowed = if read_back { &f.slow_get_ms } else { &f.slow_ms };
    slowed.set(slow);
    let press = Touch { button: true, ..Touch::finger(300, 300) };
    let (start, mut x, mut ups) = (mk_uptime_ms(), 302, 0);
    while mk_uptime_ms() < start + 3_000 {
        for dx in [1, 2, 3] {
            if !read_back {
                f.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
            }
            f.pad.lock().push_input(&[3, 0, dx, 0]);
            poll(&mut s);
        }
        x += 1;
        f.pad.lock().push_input(&touch_report(&Touch { x, ..press }));
        poll(&mut s);
        ups += of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
        mk_idle_ms(8);
    }
    ups
}

#[test]
fn a_set_up_stalling_150_or_500_ms_while_the_stick_moves_keeps_the_drag() {
    for (slow, read_back) in [(150, true), (500, true), (150, false), (500, false)] {
        let ups = stall(slow, read_back);
        assert_eq!(ups, 0, "a {slow} ms set-up let go (mode read back: {read_back})");
    }
}
