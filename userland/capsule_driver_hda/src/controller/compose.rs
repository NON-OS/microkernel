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

//! The 32-bit command word a codec verb travels in (HDA 1.0a section 7.1.2).

/// Codec at 31:28, node at 26:20, a twelve-bit verb at 19:8, payload at 7:0.
pub(crate) const fn compose_verb(codec: u8, node: u8, verb: u16, payload: u16) -> u32 {
    ((codec as u32 & 0x0f) << 28)
        | ((node as u32 & 0x7f) << 20)
        | ((verb as u32 & 0x0fff) << 8)
        | (payload as u32 & 0xff)
}

/// The four-bit verbs: verb at 19:16 and a sixteen-bit payload at 15:0.
pub(crate) const fn compose_verb_long(cad: u8, nid: u8, verb4: u16, payload16: u16) -> u32 {
    ((cad as u32 & 0x0f) << 28)
        | ((nid as u32 & 0x7f) << 20)
        | ((verb4 as u32 & 0x0f) << 16)
        | (payload16 as u32)
}
