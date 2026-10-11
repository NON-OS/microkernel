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

//! The SD clock divider (SDHCI 3.0, 2.2.14), chosen as Linux's
//! sdhci_calc_clk does for a host without preset values or a programmable
//! clock: the fastest clock not above the target.

use super::regs::SPEC_300;

/// Identification runs at 400 kHz or less (JEDEC eMMC 5.1, 7.4).
pub const IDENT_HZ: u32 = 400_000;
/// Backward-compatible (legacy) timing: 0 to 26 MHz. 20 MHz leaves margin.
pub const LEGACY_HZ: u32 = 20_000_000;
/// High Speed SDR at 26 MHz, for a device that offers only that.
pub const HS26_HZ: u32 = 26_000_000;
/// High Speed SDR at 52 MHz.
pub const HS52_HZ: u32 = 52_000_000;

/// Most a version 3 divider names: the 10-bit N in f = base / 2N.
pub const MAX_N_V3: u32 = 1023;
/// Most a version 1/2 divider divides by: 256.
pub const MAX_DIV_V2: u32 = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Divider {
    /// The divider bits of Clock Control: N 7:0 in 15:8, N 9:8 in 7:6.
    pub field: u16,
    /// The SD clock it gives, in Hz.
    pub hz: u32,
}

/// The divider for `target_hz` from `base_hz` under specification version
/// `spec`. Version 3.00 and later divide by 2N for N in 1..=1023, or by one
/// for N = 0. Earlier versions divide by a power of two from 1 to 256, the
/// field holding the divisor halved. Never above the target unless the
/// target is below the slowest clock the divider reaches.
pub fn divider(base_hz: u32, target_hz: u32, spec: u8) -> Divider {
    if spec >= SPEC_300 {
        if base_hz <= target_hz {
            return Divider { field: 0, hz: base_hz };
        }
        let mut n = 1u32;
        while n < MAX_N_V3 && base_hz / (2 * n) > target_hz {
            n += 1;
        }
        let field = (((n & 0xff) << 8) | (((n >> 8) & 0x3) << 6)) as u16;
        return Divider { field, hz: base_hz / (2 * n) };
    }
    let mut div = 1u32;
    while div < MAX_DIV_V2 && base_hz / div > target_hz {
        div *= 2;
    }
    Divider { field: ((div >> 1) << 8) as u16, hz: base_hz / div }
}
