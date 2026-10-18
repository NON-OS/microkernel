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

//! What scanning, pairing and forgetting keys ask, and that nothing is
//! asked of a service that is not there.

use super::fixture::{answered, two_bonds, KB, MOUSE};
use crate::bluetooth::panel::key::BtKey;
use crate::bluetooth::panel::{act, Panel, Request, Step};
use nonos_bt_proto::ops::{IDLE, OP_FORGET, OP_PAIR, OP_SCAN, SCANNING};
use nonos_bt_proto::{forget_body, Link};

fn asks(p: &mut Panel, key: BtKey) -> Request {
    match act(p, key) {
        Step::Ask(r) => r,
        other => panic!("{key:?} asked nothing: {other:?}"),
    }
}

#[test]
fn s_turns_scanning_the_other_way() {
    let mut on = answered(SCANNING, Link::SCANNING);
    assert_eq!(asks(&mut on, BtKey::Scan), Request::new(OP_SCAN, &[0]));
    let mut off = answered(IDLE, 0);
    assert_eq!(asks(&mut off, BtKey::Scan), Request::new(OP_SCAN, &[1]));
    assert_eq!(asks(&mut off, BtKey::Pair), Request::new(OP_PAIR, &[]));
}

#[test]
fn f_forgets_the_highlighted_device_by_its_identity() {
    let mut p = answered(IDLE, 0);
    assert_eq!(act(&mut p, BtKey::Forget), Step::Shown);
    assert_eq!(p.notice, Some("No device is paired"));
    two_bonds(&mut p);
    assert_eq!(asks(&mut p, BtKey::Forget), Request::new(OP_FORGET, &forget_body(1, &KB)));
    assert_eq!(act(&mut p, BtKey::Down), Step::Shown);
    assert_eq!(asks(&mut p, BtKey::Forget).body(), forget_body(0, &MOUSE));
}

#[test]
fn the_arrows_stay_inside_the_list() {
    let mut p = answered(IDLE, 0);
    two_bonds(&mut p);
    for _ in 0..3 {
        act(&mut p, BtKey::Down);
    }
    assert_eq!(p.cursor, 1);
    for _ in 0..3 {
        act(&mut p, BtKey::Up);
    }
    assert_eq!(p.cursor, 0);
}
