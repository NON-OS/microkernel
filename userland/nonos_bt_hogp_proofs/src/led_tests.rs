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
//! The lock LEDs: a lock key going down toggles its lock, and the boot
//! keyboard output report gets the new octet (HID 1.11, B.1) in a Write
//! Command (Vol 3, Part F, 3.4.5.3).

use crate::sim::devices::{NOTIFY, READ, WRITE};
use crate::sim::{combo, keyboard_mouse, Db, Run};
use nonos_bt_hogp::report::{CAPS_LOCK, NUM_LOCK};

const CAPS: u8 = 0x39;
const NUM: u8 = 0x53;
fn key(k: u8) -> [u8; 8] {
    [0, 0, k, 0, 0, 0, 0, 0]
}

/// The writes the peripheral has had since `from`, once delivered.
fn writes_after(run: &mut Run, from: usize) -> Vec<(u16, Vec<u8>, bool)> {
    run.go();
    run.peer.writes[from..].to_vec()
}

#[test]
fn caps_lock_lights_and_darkens_the_led() {
    let db = keyboard_mouse();
    let (kb, out) = (db.handle_of(0x2a22), db.handle_of(0x2a32));
    let mut run = Run::new(db);
    run.go();
    let n = run.peer.writes.len();
    run.notify(kb, &key(CAPS));
    assert_eq!(writes_after(&mut run, n), [(out, vec![CAPS_LOCK], true)]);
    // Letting go, and a key that is no lock, change nothing.
    run.notify(kb, &[0; 8]);
    run.notify(kb, &key(0x04));
    assert_eq!(writes_after(&mut run, n + 1), []);
    run.notify(kb, &[0, 0, 0x04, NUM, 0, 0, 0, 0]);
    assert_eq!(writes_after(&mut run, n + 1), [(out, vec![CAPS_LOCK | NUM_LOCK], true)]);
    run.notify(kb, &[0; 8]);
    run.notify(kb, &key(CAPS));
    assert_eq!(writes_after(&mut run, n + 2), [(out, vec![NUM_LOCK], true)]);
    assert_eq!(run.host.locks(), NUM_LOCK);
}

#[test]
fn no_usable_output_report_means_no_writes() {
    // No output report at all, then one that takes only Write Requests.
    let only_request = Db::default()
        .service(0x1812)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .char(READ | WRITE, 0x2a32, &[0]);
    for db in [combo(), only_request] {
        let kb = db.handle_of(0x2a22);
        let mut run = Run::new(db);
        run.go();
        let n = run.peer.writes.len();
        run.notify(kb, &key(CAPS));
        assert_eq!(writes_after(&mut run, n), []);
        // The lock is still kept, for the session to read.
        assert_eq!(run.host.locks(), CAPS_LOCK);
    }
}
