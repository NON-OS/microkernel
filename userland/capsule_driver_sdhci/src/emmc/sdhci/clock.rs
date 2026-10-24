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

//! The SD clock divider. From version 3.00 the Clock Control register's
//! 10-bit N divides the base clock by 2N, or not at all for N = 0; before
//! it, an 8-bit field holds a power-of-two divisor halved (SD Host
//! Controller Simplified Specification, Clock Control, as recalled,
//! unverified). The divider chosen is the smallest that keeps the real SD
//! clock at or below the target, so a card is never clocked past the rate
//! it was set up for.

use super::regs::SPEC_300;

/// Identification runs at 400 kHz or less (JESD84, as recalled).
pub const IDENT_HZ: u32 = 400_000;
/// Backward-compatible timing allows up to 26 MHz (JESD84, as recalled);
/// 20 MHz leaves margin for a board's traces.
pub const LEGACY_HZ: u32 = 20_000_000;
/// High Speed SDR at 26 MHz, for a device that offers only that.
pub const HS26_HZ: u32 = 26_000_000;
/// High Speed SDR at 52 MHz.
pub const HS52_HZ: u32 = 52_000_000;

/// The largest N a version 3 field holds.
pub const MAX_N_V3: u32 = 1023;
/// The largest divisor before version 3.
pub const MAX_DIV_V2: u32 = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Divider {
    /// The Clock Control divider bits: N 7:0 in 15:8, N 9:8 in 7:6.
    pub field: u16,
    /// The SD clock it gives, in Hz.
    pub hz: u32,
}

/// The divider for `target_hz` from `base_hz` under specification version
/// `spec`; the slowest the field allows when even that is above target.
pub fn divider(base_hz: u32, target_hz: u32, spec: u8) -> Divider {
    let target = target_hz.max(1);
    if spec >= SPEC_300 {
        if base_hz <= target {
            return Divider { field: 0, hz: base_hz };
        }
        // base / 2N <= target  <=>  N >= base / (2 * target), rounded up.
        let n = base_hz.div_ceil(2 * target).clamp(1, MAX_N_V3);
        let field = ((n & 0xff) << 8 | (n >> 8 & 0x3) << 6) as u16;
        return Divider { field, hz: base_hz / (2 * n) };
    }
    let div = base_hz.div_ceil(target).next_power_of_two().clamp(1, MAX_DIV_V2);
    Divider { field: ((div >> 1) << 8) as u16, hz: base_hz / div }
}
