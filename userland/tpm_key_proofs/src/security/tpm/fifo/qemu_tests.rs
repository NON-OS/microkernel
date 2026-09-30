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

//! The shipping FIFO protocol against the status register of QEMU's `tpm-tis`,
//! which clears stsValid while idle or ready and sets it with the first command
//! byte: a driver waiting on stsValid before sending would stall every command.

use super::bus::FifoBus;
use super::harness::{command, frame};
use super::model::{Model, State};
use super::regs::{STS_VALID, TPM_STS};
use super::session::run_command;

struct QemuTis(Model);

impl QemuTis {
    fn quiet(&self) -> bool {
        matches!(self.0.state, State::Idle | State::Ready)
    }
}

impl FifoBus for QemuTis {
    fn read8(&mut self, offset: u32) -> u8 {
        let value = self.0.read8(offset);
        if offset == TPM_STS && self.quiet() {
            return value & !STS_VALID;
        }
        value
    }

    fn read32(&mut self, offset: u32) -> u32 {
        let value = self.0.read32(offset);
        if offset == TPM_STS && self.quiet() {
            return value & !(STS_VALID as u32);
        }
        value
    }

    fn write8(&mut self, offset: u32, value: u8) {
        self.0.write8(offset, value);
    }

    fn now_ms(&mut self) -> u64 {
        self.0.now_ms()
    }
}

#[test]
fn a_part_without_stsvalid_while_ready_still_runs_every_command() {
    for (len, burst) in [(12, 1), (45, 3), (300, 64), (300, 4096)] {
        let response = frame(19, 9);
        let mut model = Model::new(response.clone());
        model.burst = burst;
        let mut part = QemuTis(model);
        let mut out = [0u8; 64];
        assert_eq!(run_command(&mut part, &command(len), &mut out), Ok(19), "len {len}");
        assert_eq!(&out[..19], &response[..]);
        assert_eq!(part.0.ran, command(len));
        assert!(!part.0.active, "locality kept");
        assert!(part.0.violations.is_empty(), "{:?}", part.0.violations);
    }
}
