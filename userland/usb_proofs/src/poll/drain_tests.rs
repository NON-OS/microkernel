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

//! One endpoint's read in the poll loop: asked only when its pace says it is
//! due, and at most REPORTS_PER_POLL reports at a time. It was asked on
//! every pass, and read for as long as the device had another report ready,
//! which a device that always had one held the loop on for good.

use super::drain_endpoint::drain_endpoint;
use super::pace::Pace;
use crate::descriptors::HidKind;
use crate::endpoints::ep;
use crate::state::State;
use crate::xhci::interrupt::{asks, clear, queue};

const KEY_A: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];

#[test]
fn an_endpoint_is_asked_only_when_due() {
    nonos_libc::reset();
    clear();
    let (mut state, mut buf) = (State::new(), [0u8; 64]);
    let mut kbd = ep(1, 1, 3, HidKind::Keyboard);
    kbd.pace = Pace::new(8);
    let (mut asked, mut read) = (Vec::new(), Vec::new());
    for t in 5_000..5_040 {
        if t == 5_001 {
            queue(3, &KEY_A);
        }
        let before = asks();
        if drain_endpoint(&mut state, &mut kbd, &mut buf, t) {
            read.push(t);
        }
        if asks() > before {
            asked.push(t);
        }
    }
    assert_eq!(asked, [5_000, 5_008, 5_016, 5_024, 5_032]);
    assert_eq!((read, state.key_reports), (vec![5_008], 1), "read when next due");
}

#[test]
fn a_device_with_reports_always_ready_is_read_sixteen_at_a_time() {
    nonos_libc::reset();
    clear();
    let (mut state, mut buf) = (State::new(), [0u8; 64]);
    let mut mouse = ep(1, 1, 5, HidKind::Mouse);
    for _ in 0..40 {
        queue(5, &[0, 1, 1]);
    }
    assert!(drain_endpoint(&mut state, &mut mouse, &mut buf, 1));
    assert_eq!((asks(), state.mouse_reports), (16, 16));
    assert!(drain_endpoint(&mut state, &mut mouse, &mut buf, 2));
    assert_eq!(state.mouse_reports, 32, "the next pass reads on");
}
