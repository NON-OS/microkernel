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

//! The SDA hold time, which the core needs as well as the SCL counts.

/// IC_SDA_HOLD: SDA held 300 ns after SCL falls when transmitting (bits
/// 15:0), and one input clock when receiving (bits 23:16), so the receive
/// hold is never zero (UM10204's hold minimum, as recalled, unverified).
pub fn sda_hold(clk_hz: u32) -> u32 {
    const HOLD_NS: u64 = 300;
    let ticks = (clk_hz as u64).saturating_mul(HOLD_NS) / 1_000_000_000;
    ticks.clamp(1, u16::MAX as u64) as u32 | (1 << crate::constants::IC_SDA_HOLD_RX_SHIFT)
}
