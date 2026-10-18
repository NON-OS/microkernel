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

//! What reads and writes share: the card address of a block, the end of
//! a data phase in its command buffer, and the phase run to its end
//! (rtsx_pci_dma_transfer). The chip raises TRANS_FAIL when the SD engine
//! ends a transfer in an error, as Linux relies on; SD_TRANSFER read back
//! after the check fails the transfer all the same should it not.

use super::info::Card;
use super::limits::DATA_MS;
use crate::engine::{clear_error, results, start, start_dma, stop, wait};
use crate::error::{Result, RtsxError};
use crate::regs::sd::{SD_TRANSFER, SD_TRANSFER_END, SD_TRANSFER_ERR};
use crate::setup::Driver;
use crate::wire::{CmdBuf, CmdKind};

/// What the data buffer holds: 128 blocks of 512 bytes.
pub const MAX_BLOCKS: u16 = 128;

/// The argument that names block `lba`: the block itself on a high
/// capacity card, its byte offset on a standard one.
pub fn address(card: &Card, lba: u32) -> Result<u32> {
    if card.high_capacity {
        return Ok(lba);
    }
    lba.checked_mul(512).ok_or(RtsxError::TooLarge)
}

/// The last two entries of a data phase's buffer, which has no other read
/// or check: the check that the SD engine ended, then SD_TRANSFER read.
pub fn end_entries(buf: &mut CmdBuf) {
    buf.add(CmdKind::Check, SD_TRANSFER, SD_TRANSFER_END, SD_TRANSFER_END);
    buf.add(CmdKind::Read, SD_TRANSFER, 0, 0);
}

/// The command buffer `buf` with `len` bytes moving through ADMA.
pub fn run_data(drv: &Driver, buf: &CmdBuf, len: u32, to_host: bool) -> Result<()> {
    start(drv, buf)?;
    start_dma(drv, len, to_host);
    let done = wait(drv, DATA_MS, RtsxError::DataTimeout, RtsxError::DataFailed);
    let done = done.and_then(|()| ended_clean(drv));
    if done.is_err() {
        clear_error(drv);
    }
    done
}

/// The results are the check's byte, then SD_TRANSFER's. An error bit
/// there fails the transfer, and stops the engines as a failed wait does.
fn ended_clean(drv: &Driver) -> Result<()> {
    if results(drv)[1] & SD_TRANSFER_ERR == 0 {
        return Ok(());
    }
    stop(drv);
    Err(RtsxError::DataFailed)
}
