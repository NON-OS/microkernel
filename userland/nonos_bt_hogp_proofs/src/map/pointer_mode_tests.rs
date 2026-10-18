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

//! Pointers taken to report mode, and those kept in boot mode: a service
//! is upgraded only when its map replaces each of its boot reports.

use super::maps::KEYBOARD;
use super::mice::{absolute, mouse, wide};
use crate::sim::pointer_devices::*;
use crate::sim::report_devices::{CONSUMER_IN, KEYS_IN, MODE};
use crate::sim::Run;
use nonos_bt_hogp::host::{Kind, Outcome};
use nonos_bt_hogp::report::Input::*;

pub fn kinds(run: &Run) -> Vec<(Kind, u16)> {
    run.host.sources().iter().map(|s| (s.kind, s.value)).collect()
}

#[test]
fn a_keyboard_with_a_pointer_goes_to_report_mode_whole() {
    let mut run = Run::new(keyboard_pointer(&[&KEYBOARD[..], &mouse()].concat(), &[3, 1]));
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    let report = [(Kind::Report, KEYS_IN), (Kind::Report, CONSUMER_IN), (Kind::Report, POINTER_IN)];
    assert_eq!(kinds(&run), report, "both boot reports replaced");
    assert_eq!(run.peer.writes[0], (MODE, vec![1], true));
    assert!(!run.peer.writes.iter().any(|w| w.0 == BOOT_MOUSE + 1), "the boot mouse stays off");
    let moved = run.notify(POINTER_IN, &[0x01, 0x10, 0xf0, 0xff]);
    assert_eq!(moved, [Motion { dx: 16, dy: -16 }, Wheel(-1), ButtonDown(1)]);
    assert_eq!(run.notify(BOOT_MOUSE, &[0x02, 1, 1]), []);
    assert_eq!(run.notify(KEYS_IN, &[0, 0, 0x04, 0, 0, 0, 0, 0]), [KeyDown(0x04)]);
    assert_eq!(run.host.disconnected(), [KeyUp(0x04), ButtonUp(1)]);
}

#[test]
fn a_keyboard_with_a_pointer_that_cannot_replace_both_stays_in_boot_mode() {
    let boot = |run: &mut Run| {
        assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
        assert_eq!(run.peer.writes[0], (MODE, vec![0], true));
        assert_eq!(
            kinds(run).iter().map(|k| k.0).collect::<Vec<_>>(),
            [Kind::Keyboard, Kind::Mouse]
        );
    };
    boot(&mut Run::new(keyboard_pointer(&[&KEYBOARD[..], &mouse()].concat(), &[4, 1])));
    boot(&mut Run::new(keyboard_pointer(&[&KEYBOARD[..], &absolute()].concat(), &[3, 1])));
    // A map with the pointer alone cannot type for the boot keyboard.
    boot(&mut Run::new(keyboard_pointer(&mouse(), &[3, 1])));
}

#[test]
fn a_mouse_alone_goes_to_report_mode_with_its_wheel() {
    let mut run = Run::new(mouse_alone(&wide(), &[0, 1]));
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: false, mouse: true }));
    assert_eq!(kinds(&run), [(Kind::Report, MOUSE_IN)]);
    assert_eq!(run.peer.writes[0], (MOUSE_MODE, vec![1], true));
    let [x0, x1] = 3000i16.to_le_bytes();
    let moved = run.notify(MOUSE_IN, &[0x04, x0, x1, 0, 0, 0x01]);
    assert_eq!(moved, [Motion { dx: 127, dy: 0 }, Wheel(1), ButtonDown(3)]);
}
