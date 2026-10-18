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

//! An H4 length over its kind's bound ends the stream, and one exactly at
//! the bound is waited for and taken whole (Vol 4, Part E, 5.4.2).

use nonos_bt_hci::uart::h4::H4Reader;
use nonos_bt_hci::HciError;
use nonos_bt_wire::WireError;

#[test]
fn a_length_over_its_kinds_bound_ends_the_stream() {
    let mut r = H4Reader::new(27);
    r.push(&[0x02, 0x40, 0x20, 28, 0]);
    assert_eq!(r.take(), Err(HciError::Wire(WireError::TooLong)));
    let mut r = H4Reader::new(27);
    r.push(&[0x02, 0x40, 0x20, 27, 0]);
    assert_eq!(r.take(), Ok(None), "exactly the bound waits for its octets");
    r.push(&[0u8; 27]);
    assert_eq!(r.take().unwrap().unwrap().1.len(), 31);
}
