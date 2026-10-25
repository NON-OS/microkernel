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
//! 1341), DPLL_ENABLE's lock 30 after 31 and power state 26 after 27 (p.
//! 656), PWR_WELL_CTL_DDI2's port states after their requests (p. 1074),
//! and DDI_BUF_CTL's idle 7 as the opposite of enable 31 (pp. 353, 354).

use super::regs::Regs;

/// TRANS_CONF A to D (Vol 2c p. 1340).
pub const CONFS: [u32; 4] = [0x70008, 0x71008, 0x72008, 0x73008];
/// DPLL0, DPLL1 and DPLL4 _ENABLE (p. 655).
pub const DPLL_ENABLES: [u32; 3] = [0x46010, 0x46014, 0x46018];

/// PWR_WELL_CTL_DDI2 (p. 1072) and DDI_BUF_CTL A to C (p. 352).
pub const PWR_DDI2: u32 = 0x45454;
pub const BUF_CTLS: [u32; 3] = [0x64000, 0x64100, 0x64200];

/// The (request, status, inverted) bit triples of `reg`.
pub fn pairs(reg: u32) -> &'static [(u32, u32, bool)] {
    if CONFS.contains(&reg) {
        &[(31, 30, false)]
    } else if DPLL_ENABLES.contains(&reg) {
        &[(31, 30, false), (27, 26, false)]
    } else if reg == PWR_DDI2 {
        &[(1, 0, false), (3, 2, false), (5, 4, false)]
    } else if BUF_CTLS.contains(&reg) {
        &[(31, 7, true)]
    } else {
        &[]
    }
}

/// The status bits of `reg`, which a write cannot set.
pub fn status_mask(reg: u32) -> u32 {
    pairs(reg).iter().fold(0, |m, (_, s, _)| m | (1 << s))
}

/// Moves each status bit of `reg` one read nearer its request bit.
pub(super) fn settle(r: &mut Regs, reg: u32) {
    for &(req, st, inv) in pairs(reg) {
        let v = r.get(reg);
        let want = ((v >> req) & 1) ^ inv as u32;
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
