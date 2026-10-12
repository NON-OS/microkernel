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
//! The bulk messages (CCID 1.1, 6.1 and 6.2, as recalled, unverified): a
//! 10-byte header (bMessageType, dwLength of what follows, bSlot, bSeq,
//! three bytes the message's own), then its data. The host sends
//! PC_to_RDR messages and reads RDR_to_PC ones, each answering the bSeq it
//! was sent under.

use alloc::vec::Vec;

pub const HEADER: usize = 10;
pub const ICC_POWER_ON: u8 = 0x62;
pub const ICC_POWER_OFF: u8 = 0x63;
pub const GET_SLOT_STATUS: u8 = 0x65;
pub const XFR_BLOCK: u8 = 0x6F;
pub const DATA_BLOCK: u8 = 0x80;
pub const SLOT_STATUS: u8 = 0x81;

/// A command for `slot` under `seq`; `data` after the header. Power on
/// asks for automatic voltage (bPowerSelect 0); a block waits the card's
/// own time (bBWI 0) and is one whole APDU (wLevelParameter 0).
pub fn command(kind: u8, slot: u8, seq: u8, data: &[u8]) -> Vec<u8> {
    let mut m = Vec::with_capacity(HEADER + data.len());
    m.push(kind);
    m.extend((data.len() as u32).to_le_bytes());
    m.extend([slot, seq, 0, 0, 0]);
    m.extend_from_slice(data);
    m
}

/// The message kind that answers `kind`.
pub fn answers(kind: u8) -> u8 {
    match kind {
        ICC_POWER_ON | XFR_BLOCK => DATA_BLOCK,
        _ => SLOT_STATUS,
    }
}
