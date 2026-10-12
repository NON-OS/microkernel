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

//! Devices with no battery, with two, with more than the client keeps,
//! and with a level that cannot notify.

use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{combo, Db, Run};
use nonos_bt_hogp::host::{Battery, Phase, MAX_SERVICES};
use nonos_bt_wire::att;

fn hid() -> Db {
    Db::default().service(0x1812).char(READ | NOTIFY, 0x2a22, &[0; 8]).desc(0x2902, &[0, 0])
}

#[test]
fn a_device_with_no_battery_service_is_asked_once() {
    let mut run = Run::new(combo());
    run.go();
    assert!(run.batteries.is_empty());
    let find = att::find_by_type_value(1, 0xffff, 0x2800, &[0x0f, 0x18]);
    assert_eq!(run.sent.last(), Some(&find));
    assert_eq!(run.host.phase(), Phase::Ready);
}

#[test]
fn two_batteries_one_that_cannot_notify() {
    let db = hid()
        .service(0x180f)
        .char(READ | NOTIFY, 0x2a19, &[77])
        .desc(0x2904, &[0; 7])
        .desc(0x2902, &[0, 0])
        .service(0x180f)
        .char(READ, 0x2a19, &[12])
        .desc(0x2902, &[0, 0]);
    let first = db.handle_of(0x2a19);
    let second = db.attrs.last().unwrap().handle - 1;
    let mut run = Run::new(db);
    run.go();
    let at = |instance, percent| Battery { instance, percent };
    assert_eq!(run.batteries, [at(0, 77), at(1, 12)]);
    // The Presentation Format sits before the first level's CCCD.
    assert_eq!(run.peer.writes.last(), Some(&(first + 2, vec![1, 0], false)));
    // The second is read and nothing more is asked of it: it cannot
    // notify, whatever descriptors it has.
    assert_eq!(run.sent.last(), Some(&att::read(second)));
    assert_eq!(run.notify_level(second, &[11]), Some(at(1, 11)));
}

#[test]
fn batteries_past_the_bound_are_left_unread() {
    let mut db = hid();
    for n in 0..=MAX_SERVICES as u8 {
        db = db.service(0x180f).char(READ, 0x2a19, &[n]);
    }
    let mut run = Run::new(db);
    run.go();
    let read: Vec<u8> = run.batteries.iter().map(|b| b.percent).collect();
    assert_eq!(read, (0..MAX_SERVICES as u8).collect::<Vec<_>>());
    assert_eq!(run.host.phase(), Phase::Ready);
}
