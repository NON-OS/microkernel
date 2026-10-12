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

//! Temperatures as ACPI gives them: tenths of a kelvin, for _TMP and every
//! trip point (ACPI 6.5, 11.4). 0 °C is 273.15 K, 2731.5 tenths.

/// The coldest and hottest readings believed, -50 °C and 200 °C. Firmware
/// without a sensor answers 0 or a stray value, and acting on one would
/// power the machine off or spin its fans for nothing.
pub const COLDEST: u64 = 2232;
pub const HOTTEST: u64 = 4732;

/// Tenths of a kelvin to thousandths of a degree Celsius, None outside the
/// believable range.
pub fn milli_celsius(tenths_k: u64) -> Option<i32> {
    (COLDEST..=HOTTEST).contains(&tenths_k).then(|| tenths_k as i32 * 100 - 273_150)
}

/// Back to tenths of a kelvin, rounded down, for a report.
pub fn tenths_kelvin(mc: i32) -> u64 {
    u64::try_from(mc.saturating_add(273_150) / 100).unwrap_or(0)
}
