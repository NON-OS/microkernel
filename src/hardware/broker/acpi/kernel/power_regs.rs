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
//! register when the FADT says it is supported. A driver's PIO grant and any
//! memory region keep off all four; the ACPI capsule's port windows keep off
//! the reset register only, and never set SLP_EN (`sets_sleep_enable`).

use super::super::rule::{power_reg, sleep_enable, PowerReg, SleepEnable};
use super::super::rule::{PM1_SLP_EN, SLEEP_CONTROL_SLP_EN};
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
pub(crate) fn touches(space: u8, base: u64, end: u64) -> bool {
    super::super::rule::power_touches(power_regs().iter().flatten(), space, base, end)
}

/// Whether [base, end) in `space` touches the reset register.
pub(in super::super) fn reset_touched(space: u8, base: u64, end: u64) -> bool {
    super::super::rule::power_touches(power_regs()[3].iter(), space, base, end)
}

/// Whether a port write of `value`, `width` bytes at `at`, sets SLP_EN in
/// PM1a, PM1b or sleep control.
pub(in super::super) fn sets_sleep_enable(at: u64, width: u8, value: u32) -> bool {
    let enables: [Option<SleepEnable>; 3] = match fw::parser::with_data(|d| d.fadt).flatten() {
        Some(f) => [
            sleep_enable(f.pm1a_cnt.space, f.pm1a_cnt.address, f.pm1a_cnt.bit_offset, PM1_SLP_EN),
            sleep_enable(f.pm1b_cnt.space, f.pm1b_cnt.address, f.pm1b_cnt.bit_offset, PM1_SLP_EN),
            sleep_enable(
                f.sleep_control.space,
                f.sleep_control.address,
                f.sleep_control.bit_offset,
                SLEEP_CONTROL_SLP_EN,
            ),
        ],
        None => [None; 3],
    };
    super::super::rule::sets_sleep_enable(enables.iter().flatten(), at, width, value)
}
