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
//! A service with no boot reports, read by its map, beside a boot one or alone.

use super::devices::*;
use crate::map::maps::KEYBOARD;
use crate::map::mice::{absolute, mouse};
use crate::map::pointer_mode_tests::kinds;
use crate::sim::{Db, Run};
use nonos_bt_hogp::error::HogpError;
use nonos_bt_hogp::host::{Kind, Outcome};
use nonos_bt_hogp::report::Input::*;

#[test]
fn a_pointer_in_a_service_without_boot_reports_is_read_beside_the_keyboard() {
    let db = report_only(boot_keyboard(Db::default()), &mouse(), &[3, 1], false);
    let (kb, pad, cccd) = (handles(&db, 0x2a22)[0], handles(&db, 0x2a4d)[0], handles(&db, 0x2902));
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    assert_eq!(kinds(&run), [(Kind::Keyboard, kb), (Kind::Report, pad)]);
    assert_eq!(modes(&run.peer.writes), [(3, vec![0])], "only the boot service's mode");
    assert!(run.peer.writes.iter().any(|w| w.0 == cccd[1] && w.1 == [1, 0]), "the pad notifies");
    let moved = run.notify(pad, &[0x01, 0x10, 0xf0, 0xff]);
    assert_eq!(moved, [Motion { dx: 16, dy: -16 }, Wheel(-1), ButtonDown(1)]);
    assert_eq!(run.notify(kb, &[0, 0, 0x04, 0, 0, 0, 0, 0]), [KeyDown(0x04)]);
    assert_eq!(run.host.disconnected(), [KeyUp(0x04), ButtonUp(1)]);
}

#[test]
fn its_protocol_mode_is_put_in_report_mode() {
    let db = report_only(boot_keyboard(Db::default()), &mouse(), &[3, 1], true);
    let second = handles(&db, 0x2a4e)[1];
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    assert_eq!(modes(&run.peer.writes), [(3, vec![0]), (second, vec![1])]);
}

#[test]
fn a_device_with_only_such_a_service_types() {
    let db = report_only(Db::default(), &KEYBOARD, &[1, 1], false);
    let keys = handles(&db, 0x2a4d)[0];
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
    assert_eq!(kinds(&run), [(Kind::Report, keys)]);
    assert!(modes(&run.peer.writes).is_empty(), "no Protocol Mode to write");
    assert_eq!(run.notify(keys, &[0x02, 0, 0x04, 0, 0, 0, 0, 0]), [KeyDown(0xe1), KeyDown(0x04)]);
}

#[test]
fn such_a_service_that_gives_nothing_is_left_in_boot_mode() {
    // Report 4 is not in the map, and an absolute pointer is not a mouse.
    for (map, reference) in [(mouse(), [4, 1]), (absolute(), [3, 1])] {
        let db = report_only(boot_keyboard(Db::default()), &map, &reference, true);
        let (kb, second) = (handles(&db, 0x2a22)[0], handles(&db, 0x2a4e)[1]);
        let mut run = Run::new(db);
        assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
        assert_eq!(kinds(&run), [(Kind::Keyboard, kb)]);
        assert_eq!(modes(&run.peer.writes), [(3, vec![0]), (second, vec![0])]);
        let alone = report_only(Db::default(), &map, &reference, false);
        assert_eq!(Run::new(alone).go(), Some(Outcome::Failed(HogpError::NoHid)));
    }
}
