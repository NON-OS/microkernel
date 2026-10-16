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

//! The command in flight. The command buffer is shared by every command, so
//! one that was sent and never answered may still be read by the device: the
//! next is refused (`busy`) until its reply retires it, however late, or the
//! firmware is restarted. Commands are run to their reply one at a time, so
//! one still in flight between requests went unanswered within `COMMAND_MS`:
//! the serving loop then restarts the firmware, as Linux does on a command
//! timeout.

use super::super::plan::{CMD_RING, CMD_TFDS, FIRST_TBS, FIRST_TB_SLOT, TFD_SIZE};
use super::super::region::{zero, Region};
use super::queue::CmdQueue;

impl CmdQueue {
    /// Clear the descriptor ring and the first-buffer slots, with nothing in
    /// flight: the firmware was booted again.
    pub fn init<R: Region + ?Sized>(&mut self, ctrl: &R) -> bool {
        self.write_ptr = 0;
        self.in_flight = None;
        zero(ctrl, CMD_TFDS, CMD_RING * TFD_SIZE) && zero(ctrl, FIRST_TBS, CMD_RING * FIRST_TB_SLOT)
    }

    /// A command was sent and is not yet answered.
    pub fn busy(&self) -> bool {
        self.in_flight.is_some()
    }

    /// Command `sequence` was sent.
    pub fn mark(&mut self, sequence: u16) {
        self.in_flight = Some(sequence);
    }

    /// A reply with `sequence` came: the command it answers is retired.
    pub fn retire(&mut self, sequence: u16) {
        if self.in_flight == Some(sequence) {
            self.in_flight = None;
        }
    }
}
