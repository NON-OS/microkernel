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

//! LE Meta subevents (Vol 4, Part E, 7.7.65), laid out by hand.

use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::le_meta::{split, LeConnectionComplete, LeLtkRequest};
use nonos_bt_wire::hci::reports::adv_reports;
use nonos_bt_wire::WireError;

const PEER: [u8; 6] = [0x11, 0x22, 0x33, 0x44, 0x55, 0xc6];

#[test]
fn le_connection_complete() {
    let mut p = vec![0x01, 0x00, 0x40, 0x00, 0x00, 0x01];
    p.extend_from_slice(&PEER);
    p.extend_from_slice(&[0x18, 0x00, 0x00, 0x00, 0xc8, 0x00, 0x00]);
    let (code, sub) = split(&p).unwrap();
    let c = LeConnectionComplete::parse(sub).ok().unwrap();
    assert_eq!((code, c.status, c.handle, c.role), (1, 0, 0x40, 0));
    assert_eq!((c.peer_type, c.peer), (AddrType::Random, BdAddr(PEER)));
    assert_eq!((c.interval, c.latency, c.timeout), (0x18, 0, 200));
    assert!(LeConnectionComplete::parse(&sub[..17]).is_err());
}

#[test]
fn le_ltk_request() {
    let sub = [0x40, 0x00, 1, 2, 3, 4, 5, 6, 7, 8, 0x34, 0x12];
    let r = LeLtkRequest::parse(&sub).ok().unwrap();
    assert_eq!((r.handle, r.rand, r.ediv), (0x40, [1, 2, 3, 4, 5, 6, 7, 8], 0x1234));
}

#[test]
fn advertising_reports() {
    let mut s = vec![2, 0x00, 0x02];
    s.extend_from_slice(&PEER);
    s.extend_from_slice(&[3, 0x02, 0x01, 0x06, 0xc4]);
    s.extend_from_slice(&[0x04, 0x01]);
    s.extend_from_slice(&PEER);
    s.extend_from_slice(&[0, 0xd0]);
    let r = adv_reports(&s).ok().unwrap();
    assert_eq!((r.len(), r[0].addr_type, r[0].data), (2, AddrType::Public, &[2u8, 1, 6][..]));
    assert_eq!((r[0].rssi, r[1].event_type, r[1].data.len()), (-60, 4, 0));
    assert_eq!(adv_reports(&s[..s.len() - 1]).err(), Some(WireError::Short));
    let mut long = vec![1, 0, 0];
    long.extend_from_slice(&PEER);
    long.push(32);
    assert_eq!(adv_reports(&long).err(), Some(WireError::TooLong));
}
