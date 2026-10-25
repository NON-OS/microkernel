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

//! The fake's shutdown: CC.SHN 01b seen while enabled starts it, SHST reads
//! 01b (occurring) until `shut_after` clock reads have passed and then 10b
//! (complete), or 01b for ever with `never_shut`. A disable ends it.

use super::controller::Fake;
use super::registers::register;
use crate::regs::Regs;

const CC: u32 = 0x14;
const SHN_NORMAL: u32 = 1 << 14;
const SHN_MASK: u32 = 3 << 14;
const SHST_OCCURRING: u32 = 1 << 2;
const SHST_COMPLETE: u32 = 2 << 2;

impl Fake {
    /// CSTS as the fake reads it now: all ones once gone, else RDY, CFS and
    /// SHST.
    pub(super) fn csts(&mut self, regs: Regs, en: bool, rdy: bool, cfs: bool) -> u32 {
        if self.gone {
            return u32::MAX;
        }
        u32::from(rdy) | u32::from(cfs) << 1 | self.shst(regs, en)
    }

    /// CSTS.SHST for this clock read, `en` being CC.EN as read now.
    fn shst(&mut self, regs: Regs, en: bool) -> u32 {
        let notified = register(regs, CC) & SHN_MASK == SHN_NORMAL;
        if !en || !notified {
            if !en {
                self.shut = None;
            }
            return 0;
        }
        let since = *self.shut.get_or_insert_with(|| {
            nonos_libc::record("shutdown_notified", 0);
            self.ticks
        });
        // A wait with no bound would read this for ever; fail it instead.
        assert!(self.ticks - since < 1_000, "the shutdown wait outlived every budget");
        if !self.never_shut && self.ticks - since >= self.shut_after {
            SHST_COMPLETE
        } else {
            SHST_OCCURRING
        }
    }
}
