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
//! Status bits that follow a request bit some reads after it changes, as
//! the hardware's do: TRANS_CONF's state 30 after enable 31 (Vol 2c p.
//! 1341), DPLL_ENABLE's lock 30 after enable 31 and power state 26 after
//! power enable 27 (p. 656). The addresses are stated here from the pages.

use super::regs::Regs;

/// TRANS_CONF A to D (Vol 2c p. 1340).
pub const CONFS: [u32; 4] = [0x70008, 0x71008, 0x72008, 0x73008];
/// DPLL0, DPLL1 and DPLL4 _ENABLE (p. 655).
pub const DPLL_ENABLES: [u32; 3] = [0x46010, 0x46014, 0x46018];

/// The (request, status) bit pairs of `reg`.
pub fn pairs(reg: u32) -> &'static [(u32, u32)] {
    if CONFS.contains(&reg) {
        &[(31, 30)]
    } else if DPLL_ENABLES.contains(&reg) {
        &[(31, 30), (27, 26)]
    } else {
        &[]
    }
}

/// The status bits of `reg`, which a write cannot set.
pub fn status_mask(reg: u32) -> u32 {
    pairs(reg).iter().fold(0, |m, (_, s)| m | (1 << s))
}

/// Moves each status bit of `reg` one read nearer its request bit.
pub(super) fn settle(r: &mut Regs, reg: u32) {
    for &(req, st) in pairs(reg) {
        let v = r.get(reg);
        let want = (v >> req) & 1;
        if (v >> st) & 1 == want {
            r.pending.remove(&(reg, st));
            continue;
        }
        let Some(lag) = r.lag_of(reg, st) else { continue };
        let left = r.pending.entry((reg, st)).or_insert(lag);
        if *left == 0 {
            r.pending.remove(&(reg, st));
            r.set(reg, (v & !(1 << st)) | (want << st));
        } else {
            *left -= 1;
        }
    }
}
