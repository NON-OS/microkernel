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

//! A READ or WRITE request checked against the disk before anything reaches
//! it. Pure, for the host proofs.

use crate::constants::ata::SECTOR_BYTES;
use crate::constants::buffer::MAX_SECTORS;
use crate::wire::{E_INVAL, E_MSGSIZE, E_NXIO, RW_HEADER_LEN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub lba: u64,
    pub count: u32,
}

impl Range {
    pub fn bytes(&self) -> usize {
        self.count as usize * SECTOR_BYTES
    }
}

/// The range `body` names, for a disk of `sectors`. A read carries the
/// twelve bytes alone; a write carries them and then exactly the sectors.
pub fn parse(body: &[u8], write: bool, sectors: u64) -> Result<Range, i32> {
    if body.len() < RW_HEADER_LEN {
        return Err(E_MSGSIZE);
    }
    let lba = u64::from_le_bytes(body[0..8].try_into().map_err(|_| E_MSGSIZE)?);
    let count = u32::from_le_bytes(body[8..12].try_into().map_err(|_| E_MSGSIZE)?);
    if count == 0 || count > MAX_SECTORS {
        return Err(E_INVAL);
    }
    let range = Range { lba, count };
    let want = if write { RW_HEADER_LEN + range.bytes() } else { RW_HEADER_LEN };
    if body.len() != want {
        return Err(E_MSGSIZE);
    }
    match lba.checked_add(count as u64) {
        Some(end) if end <= sectors => Ok(range),
        _ => Err(E_NXIO),
    }
}
