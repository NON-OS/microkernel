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

//! INQUIRY: the standard data with the unit's vendor and product, and the
//! Unit Serial Number page when the unit has a serial.

use super::lu::Lu;
use super::rsp::sense;
use super::scsi::{put, Sense};

pub fn inquiry(lu: &Lu, cdb: &[u8], data: Option<(u64, usize)>) -> Result<(), Sense> {
    if cdb[1] & 1 == 0 {
        let mut b = vec![0, 0, 6, 2, 31, 0, 0, 0];
        b.extend(lu.vendor);
        b.extend(lu.product);
        b.extend(b"0001");
        put(data, &b);
        return Ok(());
    }
    match (cdb[2], lu.serial) {
        (0x80, Some(s)) => {
            put(data, &[&[0, 0x80, 0, s.len() as u8][..], s].concat());
            Ok(())
        }
        // No such page, or a page the model does not have.
        _ => Err(sense(0x5, 0x24, 0)),
    }
}
