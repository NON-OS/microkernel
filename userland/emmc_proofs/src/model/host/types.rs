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

//! A command as the host saw it, a data phase, and an ADMA2 descriptor.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmdRec {
    pub index: u8,
    pub arg: u32,
    pub word: u16,
    pub mode: u16,
    pub blocks: u16,
    pub blksz: u16,
    pub hz: u32,
    pub width: u8,
    pub hs: bool,
    pub at: u64,
}

/// One data phase, as the registers and the card set it up.
pub struct Xfer {
    pub index: u8,
    pub mode: u16,
    pub blocks: u16,
    pub blksz: u16,
    pub card_reads: bool,
    pub bytes: Vec<u8>,
    pub lba: u64,
}

pub struct Desc {
    pub attr: u16,
    pub len: u32,
    pub addr: u64,
}

/// A command as the Command register write issued it.
pub struct Issued {
    pub index: u8,
    pub arg: u32,
    pub mode: u16,
    pub blocks: u16,
    pub blksz: u16,
    pub word: u16,
    pub now: u64,
}
