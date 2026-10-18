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

//! The command engine taking a buffer: entries read from DMA memory at
//! HCBAR and run (`run.rs`), or held when a test stalls the engine; and a
//! stop, which drops the buffer unless the test makes the stop slow.

use nonos_libc::{dma_read, note};

use super::reader::Reader;
use super::run::Run;
use crate::regs::host::HW_AUTO_RSP;

impl Reader {
    pub(super) fn start_cmd(&mut self, v: u32) {
        self.cmd_stopped = false;
        let len = (v & 0x00FF_FFFF) as usize;
        // A buffer the engine cannot reach is never run, and never finishes.
        let Some(bytes) = dma_read(self.hcbar as u64, len) else { return };
        let words = bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]));
        let run = Run::new(words.collect(), v & HW_AUTO_RSP != 0);
        if self.knobs.stall_cmd {
            self.run = Some(run);
            return;
        }
        self.resume(run);
    }

    pub(super) fn stop_cmd(&mut self) {
        note("stop cmd");
        self.cmd_stopped = true;
        if !self.knobs.late {
            self.run = None;
        }
    }
}
