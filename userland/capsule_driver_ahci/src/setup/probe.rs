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

use crate::choose::{holds, starts_with_magic, PLAN_LBA, PLAN_MAGIC, STORE_LBA, STORE_MAGIC};
use crate::engine::{transfer, Port};
use crate::regs::Regs;

/// Whether the port's disk carries the store header and the disk plan, in the
/// order the kernel block layer checks them: the plan is read only when the
/// store header is absent. Reads only; a sector that cannot be read counts as
/// not carrying the structure, as it does in the kernel.
pub(super) fn probe(port: &mut Port, regs: Regs) -> (bool, bool) {
    let store = carries(port, regs, STORE_LBA, STORE_MAGIC);
    let plan = !store && carries(port, regs, PLAN_LBA, PLAN_MAGIC);
    (store, plan)
}

fn carries(port: &mut Port, regs: Regs, lba: u64, magic: &[u8; 8]) -> bool {
    if !holds(port.capacity_sectors, lba) || transfer(port, regs, lba, 1, false).is_err() {
        return false;
    }
    let data = port.data.user_va() as *const u8;
    let mut head = [0u8; 8];
    for (i, b) in head.iter_mut().enumerate() {
        *b = unsafe { core::ptr::read_volatile(data.add(i)) };
    }
    starts_with_magic(&head, magic)
}
