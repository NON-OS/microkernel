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

//! The multi-link clock Linux moves a link off 6 MHz to, and the move.

use super::{multilink, skl_family, ML_LCAP, ML_LCTL};
use crate::clock::pause_ms;
use crate::controller::wait::until;
use crate::regs::Regs;

const LCTL_SCF: u32 = 0xf;
const LCTL_SPA: u32 = 1 << 16;
const LCTL_CPA: u32 = 1 << 23;

/// The link clock Linux picks from LCAP: 24, 48, 12, 96, then 192 MHz.
pub fn preferred_scf(lcap: u32) -> u32 {
    for bit in [2u32, 3, 1, 4, 5] {
        if lcap & (1 << bit) != 0 {
            return bit;
        }
    }
    0
}

/// `intel_init_lctl`: a link left on the 6 MHz clock is stopped, moved to
/// a faster one, and started again.
pub fn init_link_clock(regs: Regs, vendor: u16, device: u16, bar_size: u64) {
    if !skl_family(vendor, device) {
        return;
    }
    let Some(ml) = multilink(regs, bar_size) else { return };
    let lctl = ml + ML_LCTL;
    let val = unsafe { regs.r32(lctl) };
    if val & LCTL_SCF != 0 || ((val & LCTL_SPA) != 0) != ((val & LCTL_CPA) != 0) {
        return;
    }
    let set_power = |on: bool| {
        let v = unsafe { regs.r32(lctl) } & !LCTL_SPA;
        unsafe { regs.w32(lctl, v | if on { LCTL_SPA } else { 0 }) };
        until(1, || (unsafe { regs.r32(lctl) } & LCTL_CPA != 0) == on)
    };
    if set_power(false) {
        pause_ms(1);
        let scf = preferred_scf(unsafe { regs.r32(ml + ML_LCAP) });
        unsafe { regs.w32(lctl, (val & !LCTL_SCF & !LCTL_SPA) | scf) };
    }
    set_power(true);
    pause_ms(1);
}
