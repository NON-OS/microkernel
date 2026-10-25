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

//! The poll loop's report buffer lives as long as the driver: a report read
//! into it stayed there until the next one, so the keys a keyboard held when
//! it was pulled out sat in the driver's memory for good. It
//! is cleared once each report is handed on.

use super::drain_endpoint::drain_endpoint;
use crate::descriptors::HidKind;
use crate::endpoints::ep;
use crate::state::State;
use crate::xhci::interrupt::{clear, queue};

#[test]
fn no_report_stays_in_the_buffer_once_it_is_handed_on() {
    nonos_libc::reset();
    clear();
    let (mut state, mut buf) = (State::new(), [0u8; 64]);
    let mut kbd = ep(1, 1, 3, HidKind::Keyboard);
    // Shift and "hunter2"'s first two letters held, then the keyboard goes.
    queue(3, &[0x02, 0, 0x0b, 0x18, 0, 0, 0, 0]);
    assert!(drain_endpoint(&mut state, &mut kbd, &mut buf, 1));
    assert_eq!(buf, [0; 64], "the held keys are still in the buffer");
    let mut mouse = ep(1, 2, 5, HidKind::Mouse);
    queue(5, &[0x01, 0x10, 0xf0]);
    queue(5, &[0x00, 0x01, 0x01]);
    assert!(drain_endpoint(&mut state, &mut mouse, &mut buf, 1));
    assert_eq!((buf, state.mouse_reports), ([0; 64], 2));
}
