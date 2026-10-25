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
use crate::model::mode::{cta_1080p, timing};
use nonos_intel_xelp_display::timing::frame::frame_us;

#[test]
fn frame_of_1080p() {
    // 2200 * 1125 pixels at 148.5 MHz: 16666.67 us, rounded up.
    assert_eq!(frame_us(&cta_1080p()), Some(16_667));
}

#[test]
fn no_clock_no_frame() {
    assert_eq!(frame_us(&timing(0, [64, 32, 1, 1], [1, 5, 1, 1])), None);
}

#[test]
fn widest_fields_do_not_overflow() {
    let t = timing(1, [u32::MAX; 4], [u32::MAX; 4]);
    assert!(frame_us(&t).is_some());
}
