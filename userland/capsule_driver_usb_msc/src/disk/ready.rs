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

//! Bringing a device up and keeping its writes: TEST UNIT READY, READ
//! CAPACITY(10) or (16) and SYNCHRONIZE CACHE(10).

use nonos_libc::mk_idle_ms;

use super::bot::command;
use super::scsi_io::{sense, sense_key, whole};
use super::types::{Data, Disk};
use crate::protocol::{E_IO, E_NOMEDIUM};
use crate::scsi::{
    inquiry, parse_capacity, parse_capacity16, read_capacity10, read_capacity16, start_unit,
    test_unit_ready, CAPACITY16_DATA_LEN, CAPACITY_DATA_LEN, INQUIRY_DATA_LEN,
};
use crate::state::State;

const ILLEGAL_REQUEST: u8 = 0x05;
const NOT_READY: u8 = 0x02;
/// ASC MEDIUM NOT PRESENT: a card reader's slot with no card in it.
const MEDIUM_NOT_PRESENT: u8 = 0x3A;
/// READ CAPACITY(10)'s last LBA when the device has more blocks than it
/// can say: READ CAPACITY(16) has the count.
const LAST_LBA_TOO_BIG: u32 = u32::MAX;
/// INQUIRY tries, recovering the pipes between them: a stick fresh off its
/// reset can miss the first command.
const INQUIRY_TRIES: u32 = 5;
/// TEST UNIT READY tries: a stick reports UNIT ATTENTION once after reset,
/// and a card reader or a stick with a slow controller answers NOT READY
/// for a second or more after it is configured.
const READY_TRIES: u32 = 600;
/// The pause between tries, asleep. A yield here gave a medium still spinning
/// up no time at all on an idle machine; the tries give it thirty seconds, as
/// a USB-SATA bridge or a slow stick can take.
const READY_PAUSE_MS: u64 = 50;

/// INQUIRY first, then TEST UNIT READY until the medium is ready. Linux's
/// SCSI scan always opens with INQUIRY, and some USB storage firmware does
/// not answer anything else correctly until it has seen one. Its data is
/// not needed here: a failed status is ignored, a transport that fails ends
/// the probe. With `more_luns`, a logical unit that says it has no medium is
/// given up at once, as Linux's sd_spinup_disk does, so a card reader's empty
/// slots do not each cost the whole wait before the one with a card is asked.
pub fn unit_ready(disk: &Disk, state: &mut State, more_luns: bool) -> Result<(), i32> {
    let mut data = [0u8; INQUIRY_DATA_LEN];
    let mut asked = false;
    for _ in 0..INQUIRY_TRIES {
        if command(disk, state, inquiry(), Data::In(&mut data)).is_ok() {
            asked = true;
            break;
        }
        let _ = super::recover::reset_recovery(disk);
        let _ = mk_idle_ms(READY_PAUSE_MS);
    }
    if !asked {
        return Err(E_IO);
    }
    let mut started = false;
    for _ in 0..READY_TRIES {
        // A transport error while the medium spins up is one more not-ready
        // answer: the pipes are recovered and the next try goes on.
        match command(disk, state, test_unit_ready(), Data::None) {
            Ok((0, _)) => return Ok(()),
            Ok(_) => {
                // NOT READY on a medium that waits to be told to spin up: one
                // START UNIT, as Linux sd_spinup_disk sends.
                let said = sense(disk, state);
                if more_luns && said.is_ok_and(|s| s.asc == MEDIUM_NOT_PRESENT) {
                    return Err(E_NOMEDIUM);
                }
                if said.is_ok_and(|s| s.sense_key == NOT_READY) && !started {
                    started = true;
                    let _ = command(disk, state, start_unit(), Data::None);
                }
            }
            Err(_) => {
                // `command` has already recovered the pipes; try again.
            }
        }
        let _ = mk_idle_ms(READY_PAUSE_MS);
    }
    Err(E_IO)
}

/// The block count and block length. A device past 2 TiB of 512-byte
/// blocks reports a last LBA of 0xFFFFFFFF to READ CAPACITY(10) and is asked
/// READ CAPACITY(16); it used to be served for its first 2^32 blocks only.
pub fn capacity(disk: &Disk, state: &mut State) -> Result<(u64, u32), i32> {
    let mut raw = [0u8; CAPACITY_DATA_LEN];
    let outcome = command(disk, state, read_capacity10(), Data::In(&mut raw))?;
    whole(disk, state, outcome)?;
    let c = parse_capacity(&raw).ok_or(E_IO)?;
    if c.last_lba != LAST_LBA_TOO_BIG {
        return Ok((c.block_count(), c.block_len));
    }
    let mut raw = [0u8; CAPACITY16_DATA_LEN];
    let outcome = command(disk, state, read_capacity16(), Data::In(&mut raw))?;
    // A device may return fewer than the 32 bytes asked; the 12 that carry
    // the count and the length are all that is needed.
    match outcome {
        (0, residue) if residue as usize <= CAPACITY16_DATA_LEN - 12 => {}
        other => whole(disk, state, other)?,
    }
    parse_capacity16(&raw).ok_or(E_IO)
}

/// SYNCHRONIZE CACHE(10). A device that answers ILLEGAL REQUEST has no
/// cache it lets the host flush, and its writes are already on the medium
/// as far as it will say; that is taken as done.
pub fn sync_cache(disk: &Disk, state: &mut State) -> Result<(), i32> {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x35;
    match command(disk, state, (cdb, 10), Data::None)? {
        (0, _) => Ok(()),
        _ if sense_key(disk, state)? == ILLEGAL_REQUEST => Ok(()),
        _ => Err(E_IO),
    }
}
