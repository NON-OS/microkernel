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

//! The EC handshake against a modelled controller: bytes read and written
//! through a busy input buffer, queries in order, and a stuck EC given up on
//! within a bounded wait.

use crate::ec::{pending, query, read, write, EcError, EcPorts};
use crate::ec_model::{EcModel, PORTS};

#[test]
fn bytes_round_trip_through_a_slow_ec() {
    let mut ec = EcModel::new(3);
    ec.mem[0x40] = 0x5A;
    assert_eq!(read(&PORTS, &mut ec, 0x40), Ok(0x5A));
    assert_eq!(write(&PORTS, &mut ec, 0x41, 0xC3), Ok(()));
    assert_eq!(ec.mem[0x41], 0xC3);
    assert_eq!(read(&PORTS, &mut ec, 0x41), Ok(0xC3));
    assert_eq!(ec.slept_ms, 0, "three busy reads stay within the spins");
}

#[test]
fn queries_come_in_order_and_zero_means_none() {
    let mut ec = EcModel::new(1);
    ec.events.extend([0x0A, 0x1F]);
    assert_eq!(pending(&PORTS, &mut ec), Ok(true));
    assert_eq!(query(&PORTS, &mut ec), Ok(Some(0x0A)));
    assert_eq!(query(&PORTS, &mut ec), Ok(Some(0x1F)));
    assert_eq!(pending(&PORTS, &mut ec), Ok(false));
    assert_eq!(query(&PORTS, &mut ec), Ok(None));
}

#[test]
fn a_stuck_ec_times_out_and_a_refused_port_is_an_error() {
    let mut ec = EcModel::new(0);
    ec.stuck = true;
    assert_eq!(read(&PORTS, &mut ec, 0), Err(EcError::Timeout));
    assert_eq!(ec.slept_ms, 1000, "a second of waiting, then given up");
    let elsewhere = EcPorts { data: 0x62, command: 0x67 };
    assert_eq!(read(&elsewhere, &mut EcModel::new(0), 0), Err(EcError::Io));
}
