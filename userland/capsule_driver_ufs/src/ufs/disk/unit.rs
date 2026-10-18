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

//! One logical unit as it is served: its LUN, size, and names.

use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::scsi::{capacity, inquiry, ready};

#[derive(Clone, Copy, Debug)]
pub struct Unit {
    pub lun: u8,
    pub blocks: u64,
    pub block_len: u32,
    /// INQUIRY vendor (8 bytes) then product (16), space padded.
    pub name: [u8; 24],
    pub serial: [u8; 60],
    pub serial_len: usize,
}

/// Ready, sized and named; a unit that is not ready, or whose blocks are an
/// odd size, is not served.
pub fn open<H: Hw>(hw: &H, c: &Ctx, lun: u8) -> Result<Unit, UfsError> {
    ready::wait(hw, c, lun)?;
    let (blocks, block_len) = capacity::read(hw, c, lun)?;
    let name = inquiry::name(hw, c, lun)?;
    let (serial, serial_len) = inquiry::serial(hw, c, lun);
    Ok(Unit { lun, blocks, block_len, name, serial, serial_len })
}
