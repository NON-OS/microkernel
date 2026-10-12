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

//! A battery the client cannot use ends the client, never the HID host:
//! the keyboard still types.

use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{keyboard_mouse, Db, Peripheral, Run};
use nonos_bt_hogp::host::{Battery, Outcome, Phase};
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::att::opcode::READ_REQ;

pub const KEY: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];

pub fn still_types(run: &mut Run) {
    assert!(!run.outcomes.iter().any(|o| matches!(o, Outcome::Failed(_))));
    assert_eq!(run.host.phase(), Phase::Ready);
    let kb = run.peer.db.handle_of(0x2a22);
    assert_eq!(run.notify(kb, &KEY), [Input::KeyDown(0x04)]);
}

#[test]
fn a_refused_read_leaves_no_level_and_asks_no_more() {
    let mut peer = Peripheral::new(keyboard_mouse());
    peer.refuse_op = Some((READ_REQ, 0x05));
    let mut run = Run::with(peer);
    run.go();
    assert!(run.batteries.is_empty());
    assert_eq!(run.peer.writes.len(), 3, "no CCCD write for an unread level");
    still_types(&mut run);
}

#[test]
fn a_misplaced_descriptor_ends_the_battery_only() {
    let db = Db::default()
        .service(0x1812)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .service(0x180f)
        .char(READ | NOTIFY, 0x2a19, &[64])
        .desc(0x2904, &[0; 7])
        .char(READ | NOTIFY, 0x2a1a, &[0])
        .desc(0x2902, &[0, 0]);
    let mut peer = Peripheral::new(db);
    peer.wide = true;
    let mut run = Run::with(peer);
    run.go();
    assert_eq!(run.batteries, [Battery { instance: 0, percent: 64 }]);
    // The next characteristic's CCCD came back too; it is not the level's.
    assert_eq!(run.peer.writes.len(), 1, "only the keyboard's CCCD");
    still_types(&mut run);
}
