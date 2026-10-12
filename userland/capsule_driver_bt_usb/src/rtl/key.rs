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

//! The chip's security key id, which says which signed subsections of a
//! second-layout file it takes. It is a register read with Realtek's
//! vendor command (OGF 0x3F, OCF 0x061, Vol 4, Part E, 5.4.1): five
//! parameter octets name the register, and the answer is its two octets
//! after the status, the key id first.

use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::opcode::Opcode;
use nonos_bt_wire::WireError;

pub const READ_REGISTER: Opcode = Opcode::new(0x3f, 0x061);
/// The register holding the key id, as Realtek's controllers place it.
pub const KEY_REGISTER: [u8; 5] = [0x10, 0xa4, 0xad, 0x00, 0xb0];

pub fn key_id<T: Transport>(host: &mut Host<T>) -> Result<u8, HciError> {
    match host.command(READ_REGISTER, &KEY_REGISTER)?.as_slice() {
        [key, _] => Ok(*key),
        _ => Err(HciError::Wire(WireError::Length)),
    }
}
