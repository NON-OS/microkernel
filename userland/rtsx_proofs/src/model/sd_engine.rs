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

//! The SD engine, started by a write to SD_TRANSFER: a command and its
//! response (`sd_command.rs`), or a data transfer that waits for ADMA. A
//! card that does not answer, no card, or data that did not move ends the
//! transfer with its error bit, and the chip raises TRANS_FAIL for it, as
//! rtsx_pci_dma_transfer relies on; a check that masks only the end bit
//! passes all the same. A test can quiet the TRANS_FAIL (`quiet_error`).

use super::knobs::Phase;
use super::linux::SD_TM_AUTO_WRITE_3;
use super::reader::Reader;
use crate::regs::host::TRANS_FAIL_INT;
use crate::regs::sd::*;

const ENDED: u8 = SD_TRANSFER_END | SD_STAT_IDLE;

impl Reader {
    pub(super) fn sd_transfer(&mut self, v: u8) {
        let ok = match v & 0x0F {
            SD_TM_CMD_RSP => self.command(),
            SD_TM_AUTO_READ_2 => self.command() && self.arm(Phase::Read),
            SD_TM_AUTO_WRITE_3 => self.slot.is_some() && self.arm(Phase::Write),
            _ => false,
        };
        let busy = self.data.is_some();
        let state = if !ok {
            SD_TRANSFER_END | SD_TRANSFER_ERR
        } else if busy {
            v
        } else {
            ENDED
        };
        self.ended(state);
    }

    fn arm(&mut self, phase: Phase) -> bool {
        self.data = Some(phase);
        true
    }

    /// Ended with the card's data moved, or with an error.
    pub(super) fn end_data(&mut self, moved: bool) {
        self.data = None;
        self.ended(if moved { ENDED } else { SD_TRANSFER_END | SD_TRANSFER_ERR });
    }

    fn ended(&mut self, state: u8) {
        self.regs.insert(SD_TRANSFER, state);
        if state & SD_TRANSFER_ERR != 0 && !self.knobs.quiet_error {
            self.pending |= TRANS_FAIL_INT;
        }
    }
}
