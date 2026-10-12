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
//! An RDR_to_PC message read: its header's slot, sequence and status,
//! and the data its dwLength names (CCID 1.1, 6.2, as recalled,
//! unverified).

use nonos_usbnet::xhci::E_IO;

use super::msg::HEADER;

/// bmCommandStatus, bits 7:6 of bStatus.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Done,
    Failed(u8),
    /// The card asked for more time; the answer comes in a later message.
    MoreTime,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reply<'a> {
    pub kind: u8,
    pub slot: u8,
    pub seq: u8,
    /// bmICCStatus, bits 1:0 of bStatus: 0 present and active, 1 present
    /// and not, 2 no card.
    pub icc: u8,
    pub outcome: Outcome,
    pub data: &'a [u8],
}

/// The reply in `r`, whose dwLength must fit what came.
pub fn reply(r: &[u8]) -> Option<Reply<'_>> {
    let h = r.get(..HEADER)?;
    let len = u32::from_le_bytes([h[1], h[2], h[3], h[4]]) as usize;
    let data = r.get(HEADER..HEADER.checked_add(len)?)?;
    let outcome = match h[7] >> 6 {
        0 => Outcome::Done,
        1 => Outcome::Failed(h[8]),
        2 => Outcome::MoreTime,
        _ => return None,
    };
    Some(Reply { kind: h[0], slot: h[5], seq: h[6], icc: h[7] & 0x3, outcome, data })
}

/// The message in `got` once it is whole; too long for `max` is an error.
pub fn whole(got: &[u8], max: usize) -> Result<Option<Reply<'_>>, i32> {
    let Some(h) = got.get(..HEADER) else { return Ok(None) };
    let len = u32::from_le_bytes([h[1], h[2], h[3], h[4]]) as usize;
    if len > max.saturating_sub(HEADER) {
        return Err(E_IO);
    }
    Ok(reply(got))
}
