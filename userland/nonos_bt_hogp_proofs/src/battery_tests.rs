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

//! The Battery Service client (BAS 1.0) on a HID device: its level read
//! once the reports are on, then kept current by notifications, and only
//! a percentage taken as a level (BAS 1.0, 3.1).

use crate::sim::{keyboard_mouse, Run};
use nonos_bt_hogp::host::{Battery, Outcome, Phase};
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::att;

const fn at(instance: u8, percent: u8) -> Option<Battery> {
    Some(Battery { instance, percent })
}

#[test]
fn the_level_is_read_after_the_reports_are_on_then_notified() {
    let db = keyboard_mouse();
    let (kb, mouse, level) = (db.handle_of(0x2a22), db.handle_of(0x2a33), db.handle_of(0x2a19));
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    assert_eq!(run.batteries, [at(0, 90).unwrap()]);
    // Typing never waits on the battery: its search follows the last
    // report's CCCD write.
    let pos = |pdu: Vec<u8>| run.sent.iter().position(|p| *p == pdu).unwrap();
    let mouse_on = pos(att::write(mouse + 2, &[1, 0], false));
    assert!(pos(att::find_by_type_value(1, 0xffff, 0x2800, &[0x0f, 0x18])) > mouse_on);
    assert!(pos(att::read(level)) > mouse_on);
    assert_eq!(run.notify_level(level, &[42]), at(0, 42));
    assert_eq!(run.notify_level(level, &[100]), at(0, 100));
    assert_eq!(run.notify_level(level, &[0]), at(0, 0));
    assert_eq!(run.notify(kb, &[0, 0, 0x04, 0, 0, 0, 0, 0]), [Input::KeyDown(0x04)]);
}

#[test]
fn only_one_octet_up_to_a_hundred_is_a_level() {
    let db = keyboard_mouse();
    let level = db.handle_of(0x2a19);
    let mut run = Run::new(db);
    run.go();
    assert_eq!(run.notify_level(level, &[101]), None);
    assert_eq!(run.notify_level(level, &[0xff]), None);
    assert_eq!(run.notify_level(level, &[50, 0]), None);
    assert_eq!(run.notify_level(level, &[]), None);
    // A level on another handle is no level.
    assert_eq!(run.notify_level(level + 1, &[50]), None);
    assert_eq!(run.host.phase(), Phase::Ready);
}

#[test]
fn a_level_before_the_reports_are_on_is_dropped() {
    let db = keyboard_mouse();
    let level = db.handle_of(0x2a19);
    let mut run = Run::new(db);
    assert_eq!(run.notify_level(level, &[50]), None);
    run.go();
    assert_eq!(run.batteries, [at(0, 90).unwrap()]);
}
