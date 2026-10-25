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

//! Read Blob Request (Vol 3, Part F, 3.4.4.5): part of a value too long
//! for one Read Response, from an offset into it.

use super::opcode::READ_BLOB_REQ;
use crate::cursor::Writer;
use alloc::vec::Vec;

pub fn read_blob(handle: u16, offset: u16) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(READ_BLOB_REQ).u16(handle).u16(offset);
    w.into_vec()
}
