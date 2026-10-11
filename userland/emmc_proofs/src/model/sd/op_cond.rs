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

//! ACMD41 SD_SEND_OP_COND: the OCR, and power up.

use super::super::card::{Out, Reply, St};
use super::card::out;
use super::SdCard;

const BUSY_DONE: u32 = 1 << 31;
const CCS: u32 = 1 << 30;

impl SdCard {
    /// ACMD41: with no window, the OCR alone; with one it shares, power up,
    /// done after `ready_after` polls. A high capacity card never finishes
    /// for a host that does not set HCS (4.2.3.1).
    pub(super) fn op_cond(&mut self, arg: u32) -> Out {
        self.acmd41_args.push(arg);
        let (window, ocr) = (arg & 0x00ff_8000, self.cfg.ocr);
        if window == 0 {
            return out(Reply::Short(ocr));
        }
        if window & ocr == 0 {
            self.state = St::Inactive;
            return out(Reply::Silent);
        }
        if self.cfg.ccs && arg & CCS == 0 {
            return out(Reply::Short(ocr));
        }
        self.polls += 1;
        if self.polls < self.cfg.ready_after {
            return out(Reply::Short(ocr));
        }
        self.state = St::Ready;
        out(Reply::Short(ocr | BUSY_DONE | if self.cfg.ccs { CCS } else { 0 }))
    }
}
