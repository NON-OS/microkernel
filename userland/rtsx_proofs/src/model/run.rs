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

//! A command buffer running: entries in order, the read and check results
//! written back over the buffer (the hardware auto response), then
//! TRANS_OK or TRANS_FAIL in BIPR. A check while the SD engine waits on a
//! data phase waits with it, so the buffer finishes only once ADMA has
//! moved the data.

use nonos_libc::dma_write;

use super::reader::Reader;
use crate::regs::host::{TRANS_FAIL_INT, TRANS_OK_INT};

pub struct Run {
    words: Vec<u32>,
    at: usize,
    results: Vec<u8>,
    auto_rsp: bool,
}

impl Run {
    pub(super) fn new(words: Vec<u32>, auto_rsp: bool) -> Self {
        Self { words, at: 0, results: Vec::new(), auto_rsp }
    }
}

impl Reader {
    pub(super) fn resume(&mut self, mut run: Run) {
        while let Some(&w) = run.words.get(run.at) {
            let reg = 0xC000 | ((w >> 16) & 0x3FFF) as u16;
            let (mask, data) = ((w >> 8) as u8, w as u8);
            match w >> 30 {
                0 => run.results.push(self.reg(reg)),
                1 => self.write_reg(reg, mask, data),
                _ if self.data.is_some() => {
                    self.run = Some(run);
                    return;
                }
                _ => {
                    let got = self.reg(reg);
                    run.results.push(got);
                    if got & mask != data {
                        return self.finish(run, false);
                    }
                }
            }
            run.at += 1;
        }
        self.finish(run, true);
    }

    fn finish(&mut self, run: Run, ok: bool) {
        // Results that cannot be written back leave the buffer unfinished.
        if run.auto_rsp && !dma_write(self.hcbar as u64, &run.results) {
            return;
        }
        self.pending |= if ok { TRANS_OK_INT } else { TRANS_FAIL_INT };
    }
}
