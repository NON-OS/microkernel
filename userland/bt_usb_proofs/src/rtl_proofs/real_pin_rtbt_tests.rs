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

//! The second-layout pins, each digest as computed here from the files at
//! commit ab23307cfe7f9366c819025ca3e4778299bc2db2, whose git blob ids
//! matched the commit's tree before the digests were taken.

use super::rtl::pin::Pin;
use super::rtl::pins_rtbt::*;

const SHA: [(&Pin, usize, &str); 6] = [
    (&RTL8822CU_FW, 37_327, "bb8b1221b62749baebfa3e60271d6d90b1db5e3a480e3255be82c9fb58fe9577"),
    (&RTL8852AU_FW, 63_352, "fb52f72e986d4c96dbf7263650d3766abcbcf10c0a4f23a065ba2c388b19c125"),
    (&RTL8852BU_FW, 129_733, "c2b0d34a4ff8a8ea136993f0dc4ba657522d84231b4094c2a53cda39afc91c57"),
    (&RTL8852BTU_FW, 128_140, "bd162fd3b70e805cf7a8098f22f4ddf5d89da58fafff97d87da1a1e2e0cbeb80"),
    (&RTL8852CU_FW, 112_953, "d1314df53b8f4282f9db57a727ba77c180bc6fd05bf2c306e960326a4ef48660"),
    (&RTL8851BU_FW, 49_760, "7fcf88c00b2548a5ff157102a35722db5e4be655dab1ce8d9bddb6df85f88f24"),
];

#[test]
fn each_second_layout_pin_is_the_published_file() {
    for (pin, len, hex) in SHA {
        let want: Vec<u8> =
            (0..32).map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap()).collect();
        assert_eq!((pin.len, &pin.sha256[..]), (len, &want[..]), "{}", pin.file);
    }
}
