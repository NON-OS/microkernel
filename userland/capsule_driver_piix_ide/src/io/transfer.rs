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

//! One bus master DMA transfer (82371AB data sheet, 2.7.3): the engine
//! stopped and pointed at the descriptor table, its latched status cleared,
//! the command written, the engine started, then polled until the disk is
//! done with the command or the engine faults, and stopped again. A
//! transfer counts only when the disk reports no error and the engine used
//! up its whole table.

use super::status::{E_IO, E_TIMEDOUT};
use super::taskfile::{Command, Taskfile};
use crate::channel::Channel;
use crate::constants::ata::{ST_BSY, ST_DF, ST_DRQ, ST_ERR};
use crate::constants::bmide::{
    bmis_drive_dma, BMIC, BMIC_START, BMIC_TO_MEMORY, BMIDTP, BMIS, BMIS_ACTIVE, BMIS_ERROR,
    BMIS_INTERRUPT,
};
use crate::constants::timing::{COMMAND_MS, DRAIN_MS};
use crate::wait;

pub fn transfer(ch: &Channel, drive: u8, tf: &Taskfile, cmd: Command, prd: u32) -> Result<(), i32> {
    if !ch.ready(drive) {
        return Err(recover(ch));
    }
    let dir = match cmd {
        Command::Read => BMIC_TO_MEMORY,
        Command::Write => 0,
    };
    ch.bm_write8(BMIC, dir);
    ch.bm_write32(BMIDTP, prd);
    clear(ch);
    tf.issue(&ch.cmd);
    ch.bm_write8(BMIC, dir | BMIC_START);
    // The disk is done with the command once it holds neither busy nor
    // data; the engine reports a bus fault on its own.
    let finished = wait::until(COMMAND_MS, || {
        ch.bm_read8(BMIS) & BMIS_ERROR != 0 || ch.alt_status() & (ST_BSY | ST_DRQ) == 0
    });
    // The engine can still be emptying its FIFO into memory when the disk
    // lets go; it goes inactive once its table is used up. One that stays
    // active had more table than the disk moved: a short transfer.
    if finished {
        let _ =
            wait::until(DRAIN_MS, || ch.bm_read8(BMIS) & (BMIS_ACTIVE | BMIS_ERROR) != BMIS_ACTIVE);
    }
    // Read before the stop, which clears active (82371AB 2.7.2, BMISX).
    let bm = ch.bm_read8(BMIS);
    ch.bm_write8(BMIC, dir);
    let st = ch.status();
    clear(ch);
    if !finished {
        return Err(recover(ch));
    }
    if bm & (BMIS_ERROR | BMIS_ACTIVE) != 0 || st & (ST_ERR | ST_DF | ST_DRQ) != 0 {
        return Err(E_IO);
    }
    Ok(())
}

/// Clear the latched error and interrupt (write one), keeping the drives'
/// DMA capable bits as they are.
fn clear(ch: &Channel) {
    let keep = ch.bm_read8(BMIS) & (bmis_drive_dma(0) | bmis_drive_dma(1));
    ch.bm_write8(BMIS, keep | BMIS_ERROR | BMIS_INTERRUPT);
}

/// A command that never finished: reset the channel, so the next request
/// does not find the disk still busy with it.
pub(super) fn recover(ch: &Channel) -> i32 {
    let _ = ch.reset();
    E_TIMEDOUT
}
