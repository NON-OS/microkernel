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

//! LE signaling (Vol 3, Part A, 4) and the connection parameter ranges a
//! central checks (Vol 3, Part A, 4.20; Vol 6, Part B, 4.5.1 and 4.5.2).

use nonos_bt_wire::l2cap::signal::{CONN_PARAM_UPDATE_REQUEST, NOT_UNDERSTOOD};
use nonos_bt_wire::l2cap::{command_reject, parse_signal, signal, ConnParams, Signal};
use nonos_bt_wire::WireError;

#[test]
fn a_signal_round_trips_and_identifier_zero_is_refused() {
    let data = [0x18, 0x00, 0x28, 0x00, 0x00, 0x00, 0xc8, 0x00];
    let frame = signal(CONN_PARAM_UPDATE_REQUEST, 7, &data).unwrap();
    assert_eq!(frame[..4], [0x12, 0x07, 0x08, 0x00]);
    let code = CONN_PARAM_UPDATE_REQUEST;
    assert_eq!(parse_signal(&frame), Ok(Signal { code, id: 7, data: &data }));
    assert_eq!(signal(code, 0, &data), Err(WireError::Reserved));
    assert_eq!(parse_signal(&[0x12, 0x00, 0x00, 0x00]), Err(WireError::Reserved));
    assert_eq!(parse_signal(&[0x12, 0x01, 0x02, 0x00, 0xaa]), Err(WireError::Short));
    assert_eq!(command_reject(9, NOT_UNDERSTOOD).unwrap(), [0x01, 0x09, 0x02, 0x00, 0x00, 0x00]);
}

#[test]
fn connection_parameters_inside_and_outside_the_ranges() {
    let p = ConnParams::parse(&[0x18, 0x00, 0x28, 0x00, 0x00, 0x00, 0xc8, 0x00]).unwrap();
    assert_eq!(p, ConnParams { interval_min: 24, interval_max: 40, latency: 0, timeout: 200 });
    assert!(p.acceptable());
    let reject = [
        ConnParams { interval_min: 5, ..p },
        ConnParams { interval_min: 41, ..p },
        ConnParams { interval_max: 0x0c81, ..p },
        ConnParams { latency: 500, ..p },
        ConnParams { timeout: 9, ..p },
        // 40 * 1.25 ms * 2 * (1 + 49) = 5 s, not under a 5 s timeout.
        ConnParams { latency: 49, timeout: 500, ..p },
    ];
    for r in reject {
        assert!(!r.acceptable(), "{r:?}");
    }
    assert!(ConnParams { latency: 48, timeout: 500, ..p }.acceptable());
    assert_eq!(ConnParams::parse(&[0; 7]), Err(WireError::Short));
}
