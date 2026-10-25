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

//! Where things sit: the command descriptor's three parts, the slot, and
//! the DMA regions the bring-up is handed.

use crate::ufs::env::DmaBuf;
use crate::ufs::facts::utp::{PRDT_ENTRY, UPIU_AREA};

pub const CMD_AT: usize = 0;
pub const RSP_AT: usize = UPIU_AREA;
pub const PRDT_AT: usize = 2 * UPIU_AREA;
/// One PRDT entry is all a request needs: the data region is contiguous.
pub const UCD_LEN: usize = PRDT_AT + PRDT_ENTRY;
/// What is read back of a response: its header, a query's value, and up to
/// 30 bytes of sense data.
pub const RSP_READ: usize = 64;
/// Every request goes in slot 0, one at a time: the server answers one
/// request before it takes the next, so a deeper queue would sit empty.
pub const SLOT: u32 = 0;

/// The transfer request list (1 KiB aligned), the task management list
/// (1 KiB aligned, never used but run with the other), the command
/// descriptor (128-byte aligned) and the data bounce buffer.
#[derive(Clone, Copy, Debug)]
pub struct Lists {
    pub utrl: DmaBuf,
    pub utmrl: DmaBuf,
    pub ucd: DmaBuf,
    pub data: DmaBuf,
}
