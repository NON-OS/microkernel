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

//! LE Data Length Extension's two commands, laid out and bounded.

use nonos_bt_wire::hci::data_length::{set_data_length, MaxDataLength};
use nonos_bt_wire::hci::data_length::{LE_READ_MAXIMUM_DATA_LENGTH, LE_SET_DATA_LENGTH};
use nonos_bt_wire::WireError;

#[test]
fn the_opcodes_are_ogf_8_ocf_0x22_and_0x2f() {
    assert_eq!((LE_SET_DATA_LENGTH.0, LE_READ_MAXIMUM_DATA_LENGTH.0), (0x2022, 0x202f));
}

#[test]
fn set_data_length_is_handle_octets_time_little_endian() {
    assert_eq!(set_data_length(0x0040, 251, 2120), Ok([0x40, 0, 0xfb, 0, 0x48, 0x08]));
    assert_eq!(set_data_length(0x0eff, 27, 328), Ok([0xff, 0x0e, 27, 0, 0x48, 0x01]));
    assert_eq!(set_data_length(0x0001, 251, 17040), Ok([1, 0, 0xfb, 0, 0x90, 0x42]));
}

#[test]
fn set_data_length_refuses_values_out_of_range() {
    let bad = Err(WireError::Reserved);
    for (h, o, t) in
        [(0x0f00, 251, 2120), (1, 26, 2120), (1, 252, 2120), (1, 251, 327), (1, 251, 17041)]
    {
        assert_eq!(set_data_length(h, o, t), bad, "{h:#x} {o} {t}");
    }
}

#[test]
fn the_maximum_is_read_and_bounded() {
    let ret = [0xfb, 0, 0x48, 0x08, 0x1b, 0, 0x48, 0x01];
    let m = MaxDataLength::parse(&ret).unwrap();
    assert_eq!((m.tx_octets, m.tx_time, m.rx_octets, m.rx_time), (251, 2120, 27, 328));
    assert_eq!(MaxDataLength::parse(&ret[..7]), Err(WireError::Short));
    for at in [0, 2, 4, 6] {
        for v in [0u16, 26, 252, 327, 17041, 0xffff] {
            let mut r = ret;
            r[at..at + 2].copy_from_slice(&v.to_le_bytes());
            let out_of_range =
                if at % 4 == 0 { !(27..=251).contains(&v) } else { !(328..=17040).contains(&v) };
            if out_of_range {
                assert_eq!(MaxDataLength::parse(&r), Err(WireError::Reserved), "{at} {v}");
            }
        }
    }
}
