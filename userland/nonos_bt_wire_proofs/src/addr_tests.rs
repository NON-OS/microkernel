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

//! Address types (Vol 6, Part B, 1.3) and random address kinds (Vol 6,
//! Part B, 1.3.2): the two most significant bits of the most significant
//! octet, which is the last on the wire.

use nonos_bt_wire::addr::{AddrType, BdAddr, RandomKind};
use nonos_bt_wire::WireError;

fn top(bits: u8) -> BdAddr {
    BdAddr([1, 2, 3, 4, 5, (bits << 6) | 0x05])
}

#[test]
fn random_kinds_follow_the_top_two_bits() {
    assert_eq!(top(0b11).random_kind(), Ok(RandomKind::Static));
    assert_eq!(top(0b01).random_kind(), Ok(RandomKind::Resolvable));
    assert_eq!(top(0b00).random_kind(), Ok(RandomKind::NonResolvable));
    assert_eq!(top(0b10).random_kind(), Err(WireError::Reserved));
}

#[test]
fn address_types_and_report_identity_types() {
    assert_eq!(AddrType::from_wire(0), Ok(AddrType::Public));
    assert_eq!(AddrType::from_wire(1), Ok(AddrType::Random));
    assert_eq!(AddrType::from_wire(2), Err(WireError::Reserved));
    assert_eq!(AddrType::from_report(2), Ok(AddrType::Public));
    assert_eq!(AddrType::from_report(3), Ok(AddrType::Random));
    assert_eq!(AddrType::from_report(4), Err(WireError::Reserved));
    assert_eq!((AddrType::Public.wire(), AddrType::Random.wire()), (0, 1));
}
