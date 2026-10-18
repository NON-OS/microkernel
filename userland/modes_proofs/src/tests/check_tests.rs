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

//! Clock limits and the range limits descriptor.

use crate::check::{clock_ok, in_range};
use crate::cvt::{cvt, Blanking};
use crate::dmt::dmt;
use nonos_edid::RangeLimits;

fn limits(v: (u32, u32), h: (u32, u32), clock: u32, width: u32) -> RangeLimits {
    RangeLimits {
        v_min_hz: v.0,
        v_max_hz: v.1,
        h_min_khz: h.0,
        h_max_khz: h.1,
        max_pixel_khz: clock,
        max_h_active: width,
    }
}

#[test]
fn the_clock_limit_is_inclusive_and_zero_is_none() {
    let t = dmt(1920, 1080, 60, false).unwrap().timing;
    assert!(clock_ok(&t, 148_500) && clock_ok(&t, 0));
    assert!(!clock_ok(&t, 148_499));
}

#[test]
fn a_mode_inside_a_monitors_limits_passes() {
    // 1080p60: 67.5 kHz lines, 60 Hz frames, 148.5 MHz.
    let t = dmt(1920, 1080, 60, false).unwrap().timing;
    assert!(in_range(&t, &limits((56, 76), (30, 83), 170_000, 0)));
    assert!(in_range(&t, &limits((60, 60), (67, 67), 148_500, 1920)), "edges hold");
}

#[test]
fn each_limit_alone_refuses() {
    let t = dmt(1920, 1080, 60, false).unwrap().timing;
    assert!(!in_range(&t, &limits((61, 76), (30, 83), 170_000, 0)), "frame rate");
    assert!(!in_range(&t, &limits((56, 76), (30, 66), 170_000, 0)), "line rate");
    assert!(!in_range(&t, &limits((56, 76), (30, 83), 140_000, 0)), "clock");
    assert!(!in_range(&t, &limits((56, 76), (30, 83), 170_000, 1912)), "width");
}

#[test]
fn a_four_k_mode_needs_a_four_k_monitor() {
    let t = cvt(3840, 2160, 60_000, Blanking::Reduced).unwrap();
    assert!(!in_range(&t, &limits((56, 76), (30, 83), 170_000, 0)));
    assert!(in_range(&t, &limits((24, 60), (30, 140), 600_000, 0)));
}
