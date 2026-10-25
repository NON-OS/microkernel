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

//! One step of the fake controller. RDY follows CC.EN `slow` clock reads
//! after it changes; a disable drops every command, queue and doorbell,
//! clears INTMS and CFS and wakes a mute admin queue; while enabled, ready
//! and well it serves what the driver submitted.

use super::controller::Fake;
use super::fresh::unwritten;
use super::registers::{register, set_register};
use super::ring::Ring;
use crate::regs::Regs;

const INTMS: u32 = 0x0c;
const CC: u32 = 0x14;
const CSTS: u32 = 0x1c;
/// ASQ and ACQ, each a 64-bit register.
const ADMIN_QUEUES: [u32; 2] = [0x28, 0x30];
/// SQ0 tail, CQ0 head, SQ1 tail, CQ1 head at stride 0.
const DOORBELLS: [u32; 4] = [0x1000, 0x1004, 0x1008, 0x100c];

impl Fake {
    pub(super) fn tick(&mut self, regs: Regs) {
        self.ticks += 1;
        let (en, was) = (register(regs, CC) & 1 != 0, self.en.0);
        if en && !was {
            let [sq, cq] = ADMIN_QUEUES
                .map(|r| u64::from(register(regs, r)) | u64::from(register(regs, r + 4)) << 32);
            unwritten(cq, 64, "the admin completion ring");
            self.admin = Ring::new(sq, cq, 64, false);
        }
        if !en && was {
            nonos_libc::record("disabled", 0);
            (self.admin, self.io, self.fatal, self.mute) =
                (Ring::default(), Ring::default(), false, false);
            (self.pending, self.asked) = (Vec::new(), Vec::new());
            set_register(regs, INTMS, 0);
            DOORBELLS.iter().for_each(|&d| set_register(regs, d, 0));
        }
        self.en = if en == was { self.en } else { (en, self.ticks) };
        let followed = self.ticks - self.en.1 >= self.slow;
        let (rdy, cfs) = ((en == followed) || self.wedged, self.fatal || self.wedged);
        self.armed_step(en, rdy);
        let csts = self.csts(regs, en, rdy, cfs);
        set_register(regs, CSTS, csts);
        if en && rdy && !self.gone && !cfs {
            self.serve(regs);
        }
    }

    fn serve(&mut self, regs: Regs) {
        while let Some(cmd) = self.admin.fetch(register(regs, DOORBELLS[0])) {
            self.ask(cmd);
        }
        while let Some(cmd) = self.io.fetch(register(regs, DOORBELLS[2])) {
            self.take(cmd);
        }
        self.answer_due();
        self.complete_due();
    }
}
