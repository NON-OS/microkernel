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

//! The model monitor's Get VCP reply, byte by byte from the layout: its
//! address, a length of eight, the reply opcode, a result (0, or 1 for no
//! such control), the code, a type byte, maximum and value high byte
//! first, and a checksum seeded with 0x50.

pub fn get_reply(controls: &[(u8, u16, u16)], code: u8) -> Vec<u8> {
    let c = controls.iter().find(|c| c.0 == code);
    let (rc, now, max) = c.map_or((1, 0, 0), |c| (0, c.1, c.2));
    let [mh, ml] = max.to_be_bytes();
    let [nh, nl] = now.to_be_bytes();
    let mut r = vec![0x6E, 0x88, 0x02, rc, code, 0x00, mh, ml, nh, nl];
    r.push(r.iter().fold(0x50u8, |a, b| a ^ b));
    r
}
