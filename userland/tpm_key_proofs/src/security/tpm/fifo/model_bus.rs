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

//! The model seen through the bus the shipping protocol is written against.

use super::bus::FifoBus;
use super::model::{Model, State};
use super::regs::{
    ACCESS_ACTIVE_LOCALITY, ACCESS_REQUEST_USE, ACCESS_VALID, TPM_ACCESS, TPM_DATA_FIFO, TPM_STS,
};

impl FifoBus for Model {
    fn read8(&mut self, offset: u32) -> u8 {
        match offset {
            TPM_ACCESS => ACCESS_VALID | if self.active { ACCESS_ACTIVE_LOCALITY } else { 0 },
            TPM_STS => self.status(),
            TPM_DATA_FIFO if self.state == State::Completion && self.pos < self.response.len() => {
                self.spend();
                self.pos += 1;
                self.response[self.pos - 1]
            }
            _ => self.wrong("read of a register with nothing to give"),
        }
    }

    fn read32(&mut self, offset: u32) -> u32 {
        assert_eq!(offset, TPM_STS, "only the status register is read whole");
        let sts = self.status() as u32;
        self.budget = match self.state {
            State::Ready | State::Reception => self.burst,
            State::Completion => self.burst.min(self.response.len() - self.pos),
            _ => 0,
        };
        sts | (self.budget as u32) << 8
    }

    fn write8(&mut self, offset: u32, value: u8) {
        match offset {
            TPM_ACCESS if value == ACCESS_REQUEST_USE => self.active = self.grant,
            TPM_ACCESS if value == ACCESS_ACTIVE_LOCALITY => self.active = false,
            TPM_STS => self.write_sts(value),
            TPM_DATA_FIFO => self.take(value),
            _ => {
                self.wrong("write the part does not decode");
            }
        }
    }

    fn now_ms(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }
}
