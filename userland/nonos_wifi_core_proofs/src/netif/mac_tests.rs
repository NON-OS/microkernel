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

//! The adapter's MAC address and link status operations, associated and not.

use crate::netif::envelope_tests::{parse, request, status};
use crate::netif::link::MockLink;
use nonos_wifi_core::netif::serve::{serve, MAX_RESPONSE};
use nonos_wifi_core::netif::wire::{HDR_LEN, OP_LINK_STATUS, OP_MAC_ADDRESS};

#[test]
fn mac_address_reports_the_station_mac_when_associated() {
    let mac = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
    let mut link = MockLink::up_with(mac);
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_MAC_ADDRESS, 7, &[]), &mut link, &mut out).unwrap();
    let (op, rid, plen) = parse(&out[..n]);
    assert_eq!((op, rid, plen), (OP_MAC_ADDRESS, 7, 10));
    assert_eq!(status(&out[..n]), 0);
    assert_eq!(&out[HDR_LEN + 4..HDR_LEN + 10], &mac);
}

#[test]
fn mac_address_is_a_no_link_error_when_unassociated() {
    let mut link = MockLink::down();
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_MAC_ADDRESS, 1, &[]), &mut link, &mut out).unwrap();
    let (op, _, plen) = parse(&out[..n]);
    assert_eq!((op, plen), (OP_MAC_ADDRESS, 10));
    assert_ne!(status(&out[..n]), 0, "no MAC yet reads as an error to net_core");
}

#[test]
fn link_status_reports_up_and_down() {
    let mut up = MockLink::up_with([0; 6]);
    let mut out = [0u8; MAX_RESPONSE];
    let n = serve(&request(OP_LINK_STATUS, 2, &[]), &mut up, &mut out).unwrap();
    let (op, _, plen) = parse(&out[..n]);
    assert_eq!((op, plen), (OP_LINK_STATUS, 5));
    assert_eq!(status(&out[..n]), 0);
    assert_eq!(out[HDR_LEN + 4], 1, "associated link reports up");

    let mut down = MockLink::down();
    serve(&request(OP_LINK_STATUS, 3, &[]), &mut down, &mut out).unwrap();
    assert_eq!(out[HDR_LEN + 4], 0, "unassociated link reports down");
}
