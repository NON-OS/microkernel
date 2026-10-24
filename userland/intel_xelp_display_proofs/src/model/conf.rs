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
//! TRANS_CONF's state bit catching up with its enable bit, `lag` reads
//! after the enable bit changed, or never.

use super::regs::Regs;

pub(super) fn settle(r: &mut Regs, reg: u32) {
    let v = r.get(reg);
    let want = (v >> 31) & 1;
    if (v >> 30) & 1 == want {
        return;
    }
    let Some(lag) = r.lag else { return };
    let left = r.pending.entry(reg).or_insert(lag);
    if *left == 0 {
        r.pending.remove(&reg);
        r.set(reg, (v & !(1 << 30)) | (want << 30));
    } else {
        *left -= 1;
    }
}
