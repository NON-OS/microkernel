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

//! A controller a predecessor left running: it died without its teardown,
//! the kernel's exit path cleared Bus Master and freed its frames, and the
//! claim reset nothing. Its admin and I/O queues and host memory buffer
//! still name those frames. Each clock read while RDY reads 1 and the
//! device may master the bus, it writes them ("dma"); once disabled, CC.EN
//! and RDY both 0, it has let them go ("stopped").

use std::cell::RefCell;
use std::rc::Rc;

use super::registers::{install, registers, set_register};
use super::Fake;
use crate::regs::Regs;

impl Fake {
    pub(super) fn armed_step(&mut self, en: bool, rdy: bool) {
        if self.armed && rdy && nonos_libc::bus_master() {
            nonos_libc::record("dma", self.ticks);
        }
        if self.armed && !en && !rdy {
            nonos_libc::record("stopped", self.ticks);
            self.armed = false;
        }
    }
}

/// How a predecessor can leave it, as CC.EN, CSTS.RDY and CSTS.CFS:
/// running; killed in its own disable, RDY not yet down; killed mid-enable,
/// RDY not yet up; fatal.
pub(crate) const LEFT: [(bool, bool, bool); 4] =
    [(true, true, false), (false, true, false), (true, false, false), (true, true, true)];

/// The register block and the controller, left as `LEFT[0]`.
pub(crate) fn left_running() -> (Regs, Rc<RefCell<Fake>>) {
    left_as(LEFT[0])
}

/// The register block and the controller, left `(en, rdy, cfs)`: CAP with
/// 64-entry queues, CAP.TO 1 and NVM, VS 1.4, CC with 64- and 16-byte
/// entries, the admin queues somewhere in the freed frames; RDY 5 clock
/// reads behind a change of CC.EN, so one that lags it follows in 5.
pub(crate) fn left_as((en, rdy, cfs): (bool, bool, bool)) -> (Regs, Rc<RefCell<Fake>>) {
    let regs = registers();
    let (cc, csts) = (0x46_0000 | u32::from(en), u32::from(rdy) | u32::from(cfs) << 1);
    let written = [(0x00, (1 << 24) | 63), (0x04, 1 << 5), (0x08, 0x10400), (0x14, cc)];
    let queues = [(0x1c, csts), (0x24, 0x003f_003f), (0x28, 0x7700_0000), (0x30, 0x7700_1000)];
    written.iter().chain(&queues).for_each(|&(off, v)| set_register(regs, off, v));
    let fake = install(regs);
    {
        let mut f = fake.borrow_mut();
        let changed = if en == rdy { 0 } else { 5 };
        (f.en, f.ticks, f.slow, f.armed, f.fatal) = ((en, changed), 5, 5, true, cfs);
    }
    (regs, fake)
}
