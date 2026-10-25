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

//! LE Directed Advertising Report (Vol 4, Part E, 7.7.65.11), laid out by
//! hand: a count, then per report the event type, the advertiser's
//! address type and address, the target's address type and address, and
//! the RSSI.

use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::directed::directed_reports;
use nonos_bt_wire::WireError;

const ADV: [u8; 6] = [0x11, 0x22, 0x33, 0x44, 0x55, 0xc6];
const TARGET: [u8; 6] = [0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0x45];

fn report(kind: u8, addr_type: u8, target_type: u8) -> Vec<u8> {
    let mut r = vec![kind, addr_type];
    r.extend_from_slice(&ADV);
    r.push(target_type);
    r.extend_from_slice(&TARGET);
    r.push(0xc4);
    r
}

#[test]
fn reports_are_read_with_both_addresses() {
    let mut s = vec![2];
    s.extend(report(0x01, 0x01, 0x01));
    s.extend(report(0x01, 0x02, 0x01));
    let r = directed_reports(&s).unwrap();
    assert_eq!((r.len(), r[0].addr_type, r[0].addr), (2, AddrType::Random, BdAddr(ADV)));
    assert_eq!((r[0].direct_addr, r[0].rssi), (BdAddr(TARGET), -60));
    assert_eq!(r[1].addr_type, AddrType::Public, "a public identity the controller resolved");
}

#[test]
fn a_reserved_event_type_or_target_type_refuses_the_list() {
    for (kind, target) in [(0x00, 0x01), (0x02, 0x01), (0x01, 0x00), (0x01, 0x02)] {
        let s = [vec![1], report(kind, 0x01, target)].concat();
        assert_eq!(directed_reports(&s), Err(WireError::Reserved), "{kind} {target}");
    }
    let s = [vec![1], report(0x01, 0x04, 0x01)].concat();
    assert_eq!(directed_reports(&s), Err(WireError::Reserved), "a reserved address type");
}

#[test]
fn a_count_that_disagrees_with_the_octets_is_refused() {
    let one = report(0x01, 0x01, 0x01);
    assert!(directed_reports(&[vec![2], one.clone()].concat()).is_err(), "one short");
    assert!(directed_reports(&[vec![1], one.clone(), vec![0]].concat()).is_err(), "one over");
    assert!(directed_reports(&[vec![1], one[..15].to_vec()].concat()).is_err(), "cut");
    assert_eq!(directed_reports(&[0]), Ok(vec![]));
    assert!(directed_reports(&[]).is_err());
}
