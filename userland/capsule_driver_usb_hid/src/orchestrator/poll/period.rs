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

//! How often the controller serves an interrupt endpoint, in ms: every 2^n
//! microframes of 125 us, n the Interval the controller driver gives its
//! endpoint context (xHCI 1.2 section 6.2.3.6, `capsule_driver_xhci`
//! contexts/interval.rs). At full and low speed bInterval is a period in
//! 1 ms frames, rounded down to a power of two and held to 1 to 128 ms; at
//! high speed and above it is n plus one. No report comes more often, so
//! the poll loop asks for one no more often.

/// The Port Speed id of high speed (xHCI 1.2 section 7.2.2.1.1, as
/// recalled, unverified).
const SPEED_HIGH: u8 = 3;

pub fn period_ms(speed: u8, b_interval: u8) -> u64 {
    let exp = if speed >= SPEED_HIGH {
        u32::from(b_interval.clamp(1, 16)) - 1
    } else {
        (u32::from(b_interval.max(1)) * 8).ilog2().clamp(3, 10)
    };
    ((125u64 << exp) / 1000).max(1)
}
