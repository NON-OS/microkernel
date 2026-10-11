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

//! What sits in a drive slot: nothing, a packet device, or an ATA disk and
//! its IDENTIFY DEVICE data. The data comes in by port I/O, a word per
//! kernel trip, once per disk at bring-up.

use super::Channel;
use crate::constants::ata::{
    CMD_IDENTIFY, IDENTIFY_WORDS, REG_COMMAND, REG_COUNT, REG_DATA, REG_LBA_HI, REG_LBA_LO,
    REG_LBA_MID, SIG_PACKET, ST_BSY, ST_DRQ, ST_ERR,
};
use crate::constants::timing::IDENTIFY_MS;
use crate::wait;

pub enum Probe {
    Absent,
    /// A CD or DVD drive: it speaks SCSI commands in packets, which this
    /// driver does not.
    Packet,
    Ata([u16; IDENTIFY_WORDS]),
}

impl Channel {
    pub fn probe(&self, drive: u8) -> Probe {
        if !self.ready(drive) {
            return Probe::Absent;
        }
        for reg in [REG_COUNT, REG_LBA_LO, REG_LBA_MID, REG_LBA_HI] {
            self.cmd.write8(reg, 0);
        }
        self.cmd.write8(REG_COMMAND, CMD_IDENTIFY);
        // An empty slot answers with a status of zero.
        if self.alt_status() == 0 {
            return Probe::Absent;
        }
        if !wait::until(IDENTIFY_MS, || {
            let s = self.alt_status();
            s & ST_BSY == 0 && s & (ST_DRQ | ST_ERR) != 0
        }) {
            return Probe::Absent;
        }
        let sig = (self.cmd.read8(REG_LBA_MID), self.cmd.read8(REG_LBA_HI));
        let status = self.status();
        if sig == SIG_PACKET {
            return Probe::Packet;
        }
        if status & ST_ERR != 0 || status & ST_DRQ == 0 {
            return Probe::Absent;
        }
        let mut words = [0u16; IDENTIFY_WORDS];
        for w in words.iter_mut() {
            *w = self.cmd.read16(REG_DATA);
        }
        let _ = self.status();
        Probe::Ata(words)
    }
}
