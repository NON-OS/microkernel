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

//! READ and WRITE: the range checked against the card (protocol/rw.rs)
//! before anything is asked of it, the blocks moved through the data
//! buffer, and the buffer wiped once they are delivered (C8).

use super::fault::Fault;
use crate::card::{read_blocks, write_blocks, Card};
use crate::protocol::{parse_rw, BLOCK_BYTES, E_MSGSIZE, RW_HEADER_LEN};
use crate::setup::Driver;

/// The blocks into `out`; the bytes the reply carries.
pub fn read(drv: &Driver, card: &Card, body: &[u8], out: &mut [u8]) -> Result<usize, Fault> {
    let span = parse_rw(body, card.blocks, false).map_err(Fault::Refused)?;
    let len = span.count as usize * BLOCK_BYTES;
    let dst = out.get_mut(..len).ok_or(Fault::Refused(E_MSGSIZE))?;
    let done = read_blocks(drv, card, span.lba, span.count).map_err(Fault::Card);
    let copied = done.is_ok() && drv.data.copy_out(dst);
    drv.data.wipe(len);
    done?;
    if !copied {
        return Err(Fault::Refused(E_MSGSIZE));
    }
    Ok(len)
}

/// The blocks after the range header onto the card; no reply payload.
pub fn write(drv: &Driver, card: &Card, body: &[u8]) -> Result<usize, Fault> {
    let span = parse_rw(body, card.blocks, true).map_err(Fault::Refused)?;
    let data = body.get(RW_HEADER_LEN..).unwrap_or(&[]);
    if !drv.data.copy_in(data) {
        return Err(Fault::Refused(E_MSGSIZE));
    }
    let done = write_blocks(drv, card, span.lba, span.count);
    drv.data.wipe(data.len());
    done.map(|_| 0).map_err(Fault::Card)
}
