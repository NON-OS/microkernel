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

//! A data phase: the descriptors walked, then the blocks moved between the
//! card and host memory, or the CRC errors a width mismatch gives.

use super::sim::Sim;
use super::types::Xfer;

impl Sim {
    pub(super) fn data(&mut self, x: Xfer) {
        let multi = x.index == 18 || x.index == 25;
        let len = self.check_data(&x, multi);
        let Xfer { mode, card_reads, bytes, lba, .. } = x;
        let now = self.now();
        let segs = match self.walk(len) {
            Some(s) => s,
            None => {
                self.int |= 1 << 15 | 1 << 25;
                return;
            }
        };
        if self.host_width() != self.dev_ref().width() || self.fail_next_data > 0 {
            if self.fail_next_data > 0 {
                self.fail_next_data -= 1;
            }
            // The card sends or takes the blocks; the host sees CRC errors.
            self.dev().end(now);
            self.int |= 1 << 15 | 1 << 21;
            return;
        }
        if card_reads {
            let mut bytes = bytes;
            if self.dev_ref().corrupts(self.host_width()) {
                for b in bytes.iter_mut().step_by(7) {
                    *b ^= 0x5a;
                }
            }
            self.copy_in(&segs, &bytes);
            self.dev().end(now);
        } else {
            let v = self.copy_out(&segs, len);
            self.dev().take_write(lba, &v, now);
        }
        if mode & 0x04 != 0 {
            // Auto CMD12.
            let _ = self.dev().command(12, 0, 0, now);
        }
        if card_reads {
            self.int |= 1 << 1;
        } else {
            self.tc_at = Some(now + 1);
        }
    }
}
