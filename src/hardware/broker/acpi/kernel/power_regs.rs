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

//! The FADT's power control registers, as the rule's `PowerReg`s: PM1a and
//! PM1b control, the hardware-reduced sleep control register, and the reset
//! register when the FADT says it is supported.

use super::super::rule::{power_reg, PowerReg};
use super::platform::fw;

/// The registers, read from the parsed FADT; none before ACPI is up.
pub(in super::super) fn power_regs() -> [Option<PowerReg>; 4] {
    let fadt = fw::parser::with_data(|d| d.fadt).flatten();
    let Some(f) = fadt else {
        return [None; 4];
    };
    let reg = |g: fw::hw::gas::Gas| power_reg(g.space, g.address, g.bit_offset, g.bit_width);
    [
        reg(f.pm1a_cnt),
        reg(f.pm1b_cnt),
        reg(f.sleep_control),
        if f.has_reset_reg() { reg(f.reset_reg) } else { None },
    ]
}

/// Whether [base, end) in `space` touches one of them.
pub(in super::super) fn touches(space: u8, base: u64, end: u64) -> bool {
    super::super::rule::power_touches(power_regs().iter().flatten(), space, base, end)
}
