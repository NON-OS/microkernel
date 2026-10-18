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

//! SMP PDUs laid out by hand from Vol 3, Part H, 3.5.

use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pdu::{self, auth, keys, Pairing};
use nonos_bt_wire::WireError;

fn ours() -> Pairing {
    let auth_req = auth::BONDING | auth::MITM | auth::SC;
    let (init_keys, resp_keys) = (keys::ID, keys::ENC | keys::ID);
    let io_cap = IoCap::KeyboardDisplay;
    Pairing { io_cap, oob: false, auth_req, max_key_size: 16, init_keys, resp_keys }
}

#[test]
fn this_hosts_pairing_request() {
    let req = pdu::frame(pdu::PAIRING_REQUEST, &ours().params()).unwrap();
    assert_eq!(req, [0x01, 0x04, 0x00, 0x0d, 0x10, 0x02, 0x03]);
    let (code, params) = pdu::parse(&req).unwrap();
    assert_eq!((code, Pairing::parse(params).unwrap()), (pdu::PAIRING_REQUEST, ours()));
    assert_eq!(ours().f6_io_cap(), [0x0d, 0x00, 0x04]);
}

#[test]
fn bad_pairing_parameters_are_refused() {
    for bad in [[0x05, 0, 0x0d, 16, 0, 0], [0x04, 2, 0x0d, 16, 0, 0], [0x04, 0, 0x0d, 6, 0, 0]] {
        assert_eq!(Pairing::parse(&bad), Err(WireError::Reserved));
    }
    assert_eq!(Pairing::parse(&[0x04, 0, 0x0d, 17, 0, 0]), Err(WireError::Reserved));
}

#[test]
fn lengths_are_exact_and_unknown_codes_refused() {
    assert_eq!(pdu::parse(&[pdu::PAIRING_FAILED, 0x04]), Ok((0x05, &[0x04][..])));
    assert_eq!(pdu::parse(&[pdu::PAIRING_FAILED]), Err(WireError::Short));
    assert_eq!(pdu::parse(&[pdu::PAIRING_FAILED, 4, 4]), Err(WireError::Length));
    assert_eq!(pdu::parse(&[0x0f]), Err(WireError::Reserved));
    assert_eq!(pdu::parse(&[]), Err(WireError::Short));
    assert_eq!(pdu::frame(pdu::PAIRING_PUBLIC_KEY, &[0; 63]), Err(WireError::Length));
    assert_eq!(pdu::frame(pdu::PAIRING_PUBLIC_KEY, &[0; 64]).unwrap().len(), 65);
}

#[test]
fn air_order_is_reversed_function_order() {
    let air = [1u8, 2, 3, 4];
    assert_eq!(pdu::be(&air), [4, 3, 2, 1]);
    assert_eq!(pdu::le(&pdu::be(&air)), air);
}
