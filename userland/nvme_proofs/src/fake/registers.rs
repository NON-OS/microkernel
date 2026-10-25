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

//! A register block in host memory. The driver's writes land in it and the
//! fake controller's in turn, so each reads what the other wrote.

use std::cell::RefCell;
use std::rc::Rc;

use super::controller::Fake;
use crate::controller::ControllerInfo;
use crate::regs::Regs;

/// Bytes of block: the doorbells of queues 0 and 1 at stride 0 lie below.
const BLOCK_BYTES: usize = 0x2000;

/// A zeroed register block, never freed, and the one a BAR0 map returns.
pub(crate) fn registers() -> Regs {
    let block = Box::into_raw(Box::new([0u32; BLOCK_BYTES / 4]));
    nonos_libc::set_bar0(block as u64);
    Regs::new(block as u64)
}

/// A fake controller on `regs`, run on every clock read of 100 ms each.
pub(crate) fn install(regs: Regs) -> Rc<RefCell<Fake>> {
    let fake = Rc::new(RefCell::new(Fake::default()));
    let runs = fake.clone();
    nonos_libc::set_clock(100);
    nonos_libc::on_tick(move || runs.borrow_mut().tick(regs));
    fake
}

/// Register `off`, as last written.
pub(crate) fn register(regs: Regs, off: u32) -> u32 {
    assert!((off as usize) < BLOCK_BYTES, "register {off:#x} is past the block");
    // SAFETY: every Regs the tests use is a block from `registers`, and the
    // offset lies inside it (the assert above).
    unsafe { regs.r32(off) }
}

/// Write register `off`, as the controller does.
pub(crate) fn set_register(regs: Regs, off: u32, value: u32) {
    assert!((off as usize) < BLOCK_BYTES, "register {off:#x} is past the block");
    // SAFETY: as for `register`.
    unsafe { regs.w32(off, value) }
}

/// The registers as bring-up reads them: CAP.TO 1 (500 ms, so the 5 s
/// floor), the NVM command set (CAP.CSS bit 37), 4 KiB pages, doorbell
/// stride 0, NVMe 1.4.
pub(crate) fn info() -> ControllerInfo {
    let (cap, version) = ((1 << 24) | (1 << 37), 0x10400);
    let (cc, csts, aqa, intms, intmc, cmbloc, cmbsz) = (0, 0, 0, 0, 0, 0, 0);
    ControllerInfo { cap, version, cc, csts, aqa, intms, intmc, cmbloc, cmbsz }
}
