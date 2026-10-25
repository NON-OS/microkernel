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

//! A central's answers on the LE signaling channel (Vol 3, Part A, 4.1,
//! 4.20 and 4.21), laid out byte by byte.

use nonos_bt_wire::l2cap::signal::*;
use nonos_bt_wire::l2cap::{answer_central, Answer, ConnParams};
use nonos_bt_wire::WireError;

const TIMING: [u8; 8] = [0x18, 0x00, 0x28, 0x00, 0x00, 0x00, 0xc8, 0x00];

#[test]
fn a_parameter_update_in_range_is_accepted_and_handed_on() {
    let a = answer_central(&signal(CONN_PARAM_UPDATE_REQUEST, 3, &TIMING).unwrap()).unwrap();
    assert_eq!(a.reply.unwrap(), [0x13, 0x03, 0x02, 0x00, 0x00, 0x00]);
    let p = ConnParams { interval_min: 24, interval_max: 40, latency: 0, timeout: 200 };
    assert_eq!(a.update, Some(p));
    let mut bad = TIMING;
    bad[6] = 0x09;
    let a = answer_central(&signal(CONN_PARAM_UPDATE_REQUEST, 4, &bad).unwrap()).unwrap();
    assert_eq!(a.reply.unwrap(), [0x13, 0x04, 0x02, 0x00, 0x01, 0x00]);
    assert_eq!(a.update, None);
}

#[test]
fn other_requests_are_rejected_and_rejects_are_not_answered() {
    let credit = signal(LE_CREDIT_CONNECTION_REQUEST, 5, &[0; 10]).unwrap();
    let a = answer_central(&credit).unwrap();
    assert_eq!(a.reply.unwrap(), [0x01, 0x05, 0x02, 0x00, 0x00, 0x00]);
    let short = signal(CONN_PARAM_UPDATE_REQUEST, 6, &TIMING[..6]).unwrap();
    assert_eq!(
        answer_central(&short).unwrap().reply.unwrap(),
        [0x01, 0x06, 0x02, 0x00, 0x00, 0x00]
    );
    let reject = command_reject(7, NOT_UNDERSTOOD).unwrap();
    assert_eq!(answer_central(&reject), Ok(Answer::default()));
    assert_eq!(answer_central(&[0x12, 0x00, 0x00, 0x00]), Err(WireError::Reserved));
}
