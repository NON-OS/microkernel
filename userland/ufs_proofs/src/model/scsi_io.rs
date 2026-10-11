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

//! READ and WRITE, (10) and (16), on a unit's blocks; WRITE refused with
//! DATA PROTECT on a write-protected unit.

use super::lu::Lu;
use super::mem::{read, write};
use super::rsp::sense;
use super::scsi::Sense;
use crate::ufs::scsi::codes::*;

pub fn rw(lu: &mut Lu, cdb: &[u8], data: Option<(u64, usize)>) -> Result<(), Sense> {
    let (lba, n) = match cdb[0] {
        READ_10 | WRITE_10 => (
            u32::from_be_bytes(cdb[2..6].try_into().unwrap()) as u64,
            u16::from_be_bytes(cdb[7..9].try_into().unwrap()) as u64,
        ),
        _ => (
            u64::from_be_bytes(cdb[2..10].try_into().unwrap()),
            u32::from_be_bytes(cdb[10..14].try_into().unwrap()) as u64,
        ),
    };
    if lba.checked_add(n).is_none_or(|end| end > lu.blocks) {
        return Err(sense(0x5, 0x21, 0));
    }
    let bl = lu.block_len as usize;
    let Some((addr, len)) = data else { return Err(sense(0x5, 0x24, 0)) };
    if len != n as usize * bl {
        return Err(sense(0x5, 0x24, 0));
    }
    let writing = matches!(cdb[0], WRITE_10 | WRITE_16);
    if writing && lu.write_protect {
        return Err(sense(0x7, 0x27, 0));
    }
    for i in 0..n {
        let at = addr + i * bl as u64;
        if writing {
            let mut b = vec![0u8; bl];
            unsafe { read(at, &mut b) };
            lu.data.insert(lba + i, b);
        } else {
            unsafe { write(at, &lu.block(lba + i)) };
        }
    }
    Ok(())
}
