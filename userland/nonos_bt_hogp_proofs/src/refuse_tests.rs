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

//! Peripherals this host stops on, each with the reason it gives.

use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{keyboard_mouse, Db, Peripheral, Run};
use nonos_bt_hogp::host::{Outcome, Phase};
use nonos_bt_hogp::HogpError;

pub fn failed(run: &mut Run, e: HogpError) {
    assert_eq!(run.go(), Some(Outcome::Failed(e)));
    assert_eq!(run.host.phase(), Phase::Failed);
    assert!(run.peer.writes.is_empty(), "nothing is written before discovery ends");
}

#[test]
fn no_hid_service_or_no_notifying_boot_report() {
    failed(&mut Run::new(Db::default().service(0x1800).char(READ, 0x2a00, b"x")), HogpError::NoHid);
    let quiet = Db::default().service(0x1812).char(READ, 0x2a22, &[0; 8]).desc(0x2902, &[0, 0]);
    failed(&mut Run::new(quiet), HogpError::NoHid);
}

#[test]
fn a_boot_report_without_a_cccd() {
    let db = Db::default().service(0x1812).char(READ | NOTIFY, 0x2a22, &[0; 8]).service(0x180f);
    failed(&mut Run::new(db), HogpError::NoCccd);
    let db =
        Db::default().service(0x1812).char(READ | NOTIFY, 0x2a33, &[0; 3]).desc(0x2908, &[0, 1]);
    failed(&mut Run::new(db), HogpError::NoCccd);
}

#[test]
fn a_link_without_encryption_asks_for_pairing() {
    for code in [0x05, 0x0f, 0x0c] {
        let mut peer = Peripheral::new(keyboard_mouse());
        peer.refuse = Some(code);
        failed(&mut Run::with(peer), HogpError::Security(code));
    }
    let mut peer = Peripheral::new(keyboard_mouse());
    peer.refuse = Some(0x0e);
    failed(&mut Run::with(peer), HogpError::Att(0x0e));
}
