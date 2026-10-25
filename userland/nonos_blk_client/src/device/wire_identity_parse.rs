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

//! The block wire's IDENTIFY reply (op 6), held to the wire's rules before
//! it names a disk or gives the erase-confirm word. Pure, for the host
//! proofs.

use super::identity_parse::Identity;

pub const OP_WIRE_IDENTIFY: u16 = 6;
pub const WIRE_IDENTIFY_LEN: usize = 64;
const MODEL_AT: usize = 4;
const SERIAL_AT: usize = 44;
const MEDIUM_EMMC: u8 = 1;

/// `None` for a body that is not exactly 64 bytes, names a length past its
/// field, sets the reserved byte, or holds anything but printable ASCII
/// inside a length or anything but zero past one: such a name is neither
/// shown nor typed.
pub fn parse_wire_identity(body: &[u8]) -> Option<Identity> {
    if body.len() != WIRE_IDENTIFY_LEN || body[3] != 0 {
        return None;
    }
    let model = field::<40>(&body[MODEL_AT..MODEL_AT + 40], body[0] as usize)?;
    let serial = field::<20>(&body[SERIAL_AT..SERIAL_AT + 20], body[1] as usize)?;
    Some(Identity { model, serial, emmc: body[2] == MEDIUM_EMMC })
}

fn field<const N: usize>(raw: &[u8], len: usize) -> Option<[u8; N]> {
    if len > N {
        return None;
    }
    let (text, pad) = raw.split_at(len);
    if !text.iter().all(|c| (0x20..=0x7e).contains(c)) || pad.iter().any(|&c| c != 0) {
        return None;
    }
    let mut out = [0u8; N];
    out.copy_from_slice(raw);
    Some(out)
}
