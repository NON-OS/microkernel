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
//! A combo port's clock from a DPLL: Vol 12's step 11 (p. 177, 178) and
//! the disable sequence's step 1 (p. 178), in DPCLKA_CFGCR0.

use nonos_intel_display_hw::access::{rmw, Mmio};

use crate::regs::clka::{clock_off, select, select_value, Combo, DPCLKA_CFGCR0};
use crate::regs::pll::Dpll;

/// Points `port` at `d`, then turns its clock on. Vol 12 asks for the two
/// as separate writes (p. 178), and the select is made while the clock is
/// still off.
pub fn map(m: &mut dyn Mmio, port: Combo, d: Dpll) {
    let (mask, shift) = select(port);
    rmw(m, DPCLKA_CFGCR0, mask, select_value(d) << shift);
    rmw(m, DPCLKA_CFGCR0, clock_off(port), 0);
}

/// Turns `port`'s clock off, leaving its select as it was.
pub fn unmap(m: &mut dyn Mmio, port: Combo) {
    let off = clock_off(port);
    rmw(m, DPCLKA_CFGCR0, off, off);
}
