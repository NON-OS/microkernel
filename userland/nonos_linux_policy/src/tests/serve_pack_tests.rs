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

//! The pack tool's version 2 bytes, decoded here. tools/nonos_linux_pack/
//! test_serve.py writes the same policy and holds its output to PACKED, read
//! from this file, so the tool and the crate cannot drift apart.

use crate::mask::{FILES_HOME, NET_ANON, SERVE_ONE};
use crate::{decode, market_ready, LEN, VERSION_SERVE};

/// [0x11;32], NET_ANON|FILES_HOME, [0x22;32], [0x33;32], [0x44;32], calls
/// {0,1,60,231,511}, serve app.simplex:5225, as the pack tool encodes it.
const PACKED: &str = concat!(
    "4e4c58504f4c3031020000000000000011111111111111111111111111111111",
    "111111111111111111111111111111110a020000000000002222222222222222",
    "2222222222222222222222222222222222222222222222223333333333333333",
    "3333333333333333333333333333333333333333333333334444444444444444",
    "4444444444444444444444444444444444444444444444440300000000000010",
    "0000000000000000000000000000000000000000800000000000000000000000",
    "0000000000000000000000000000000000000000000000800069146170702e73",
    "696d706c65780000000000000000000000000000000000000000000000000000",
);

#[test]
fn the_pack_tools_version_2_bytes_decode_to_its_field() {
    let b: [u8; LEN] =
        core::array::from_fn(|i| u8::from_str_radix(&PACKED[2 * i..2 * i + 2], 16).unwrap());
    let p = decode(&b).unwrap();
    assert_eq!(u16::from_le_bytes([b[8], b[9]]), VERSION_SERVE);
    assert_eq!(p.mask, NET_ANON | FILES_HOME | SERVE_ONE);
    let s = p.serve_one().unwrap();
    assert_eq!((s.port, s.name()), (5225, "app.simplex"));
    assert!(market_ready(&p));
    assert_eq!(p.encode(), b);
}
