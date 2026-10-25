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
//! Boot reports: two keyboards, or two mice, in separate services, where
//! a report from one never releases what the other holds; and a service
//! whose report mode could not be set.

use super::devices::*;
use crate::map::maps::KEYBOARD;
use crate::map::pointer_mode_tests::kinds;
use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{Db, Run};
use nonos_bt_hogp::host::{Kind, Outcome};
use nonos_bt_hogp::report::Input::*;

#[test]
fn two_boot_keyboards_keep_their_own_keys() {
    let db = boot_keyboard(boot_keyboard(Db::default()));
    let [a, b] = handles(&db, 0x2a22)[..] else { panic!("two keyboards") };
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
    assert_eq!(run.notify(a, &[0, 0, 0x04, 0, 0, 0, 0, 0]), [KeyDown(0x04)]);
    assert_eq!(run.notify(b, &[0, 0, 0x05, 0, 0, 0, 0, 0]), [KeyDown(0x05)]);
    assert_eq!(run.notify(a, &[0; 8]), [KeyUp(0x04)]);
    assert_eq!(run.notify(a, &[0, 0, 0x06, 0, 0, 0, 0, 0]), [KeyDown(0x06)]);
    assert_eq!(run.host.disconnected(), [KeyUp(0x06), KeyUp(0x05)]);
}

#[test]
fn two_boot_mice_keep_their_own_buttons() {
    let db = boot_mouse(boot_mouse(Db::default()));
    let [a, b] = handles(&db, 0x2a33)[..] else { panic!("two mice") };
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: false, mouse: true }));
    assert_eq!(run.notify(a, &[0x01, 0, 0]), [ButtonDown(1)]);
    assert_eq!(run.notify(b, &[0x02, 0, 0]), [ButtonDown(2)]);
    assert_eq!(run.notify(b, &[0, 0, 0]), [ButtonUp(2)]);
    assert_eq!(run.host.disconnected(), [ButtonUp(1)]);
}

#[test]
fn boot_reports_with_no_protocol_mode_to_switch_stay_in_boot_mode() {
    let db = Db::default()
        .service(0x1812)
        .char(READ, 0x2a4b, &KEYBOARD)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .char(READ | NOTIFY, 0x2a4d, &[0; 8])
        .desc(0x2908, &[1, 1])
        .desc(0x2902, &[0, 0]);
    let boot = handles(&db, 0x2a22)[0];
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
    assert_eq!(kinds(&run), [(Kind::Keyboard, boot)]);
    assert!(modes(&run.peer.writes).is_empty());
}
