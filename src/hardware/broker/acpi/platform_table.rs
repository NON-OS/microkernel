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

//! The platform rule on this machine: every MMIO window the device table
//! holds, and the region rule's facts for the capsule asking.

use super::super::device::BarKind;
use super::super::table;
use super::kernel::Machine;
use super::platform_rule::{admit, Verdict};
use super::platform_wire::Platform;
use alloc::vec::Vec;

fn mmio_windows() -> Vec<(u64, u64, u64)> {
    let mut out = Vec::new();
    for r in table::list() {
        let n = (r.bar_count as usize).min(r.bars.len());
        let mmio = r.bars[..n].iter().filter(|b| b.kind == BarKind::Mmio as u8 && b.size != 0);
        out.extend(mmio.map(|b| (r.device_id, b.base, b.size)));
    }
    out
}

pub fn judge(p: &Platform, pid: u32) -> Verdict {
    admit(p, &mmio_windows(), &Machine { pid })
}
