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

//! The pairing window's count, and what a request the page made did.

use super::fixture::answered;
use crate::bluetooth::panel::notice::done;
use crate::bluetooth::panel::{poll_ms, refused, window, Request};
use nonos_bt_proto::ops::*;
use nonos_bt_proto::Link;

#[test]
fn the_window_counts_down_from_this_windows_pair() {
    let mut p = answered(SCANNING, Link::WINDOW);
    assert_eq!(window(&p, 0), Some(None), "opened by another Settings window");
    p.did(&Request::new(OP_PAIR, &[]), E_OK, 1_000);
    assert_eq!(p.notice, Some(done(&Request::new(OP_PAIR, &[]))));
    assert_eq!(window(&p, 1_000), Some(Some(60)));
    assert_eq!(window(&p, 60_999), Some(Some(1)), "1 until the last moment");
    assert_eq!(poll_ms(&p), 250);
    assert!(p.took_link(&Link { state: SCANNING, flags: 0, place: 0xFF }.write()));
    assert_eq!((window(&p, 2_000), poll_ms(&p)), (None, 1000));
}

#[test]
fn a_refusal_is_said_and_an_answer_wipes_its_digits() {
    let mut p = answered(SECURING, Link::WAITING);
    assert!(p.digits.push(b'7'));
    p.did(&Request::new(OP_ANSWER, &[2, 7, 0, 0, 0]), E_INVAL, 0);
    assert_eq!((p.notice, p.digits.as_bytes()), (Some(refused(E_INVAL)), &b""[..]));
    p.did(&Request::new(OP_PAIR, &[]), E_ACCES, 0);
    assert_eq!((p.notice, p.window_from_ms), (Some("This window may not change Bluetooth"), None));
    assert_eq!(done(&Request::new(OP_SCAN, &[1])), "Scanning for keyboards and mice");
    assert_ne!(done(&Request::new(OP_SCAN, &[0])), done(&Request::new(OP_SCAN, &[1])));
}
