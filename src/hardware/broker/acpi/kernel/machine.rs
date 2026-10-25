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

//! The rule's facts on this machine, as seen by the capsule asking.

use super::super::rule::{Facts, Hardware, MemKind, SPACE_IO};
use super::{hardware, held, memmap};

/// "Claimed" means held by someone else: a capsule may reach what it holds.
pub(in super::super) struct Machine {
    pub(in super::super) pid: u32,
}

impl Facts for Machine {
    fn mem_kinds(&self, base: u64, end: u64, each: &mut dyn FnMut(MemKind)) {
        memmap::kinds(base, end, each)
    }

    fn mmio_claimed(&self, base: u64, end: u64) -> bool {
        held::mmio(self.pid, base, end)
    }

    fn protected(&self, base: u64, end: u64) -> Option<Hardware> {
        hardware::protected(base, end)
    }

    fn reset_port(&self, base: u64, end: u64) -> bool {
        super::power_regs::reset_touched(SPACE_IO, base, end)
    }

    fn ports_claimed(&self, base: u16, count: u32) -> bool {
        held::ports(self.pid, u64::from(base), u64::from(base) + u64::from(count))
    }

    fn pci_claimed(&self, bdf: (u16, u8, u8, u8)) -> bool {
        held::function(self.pid, bdf)
    }
}
