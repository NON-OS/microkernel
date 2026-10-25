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

//! Divisor arithmetic for each UART, kept free of MMIO so the host proofs
//! can check it. `None` means the clock cannot make the rate at all.

/// PL011: a 16-bit integer part and a 6-bit fraction of `clock / (16 * baud)`,
/// the fraction rounded to nearest.
pub(crate) fn pl011(clock: u32, baud: u32) -> Option<(u32, u32)> {
    let denom = 16 * baud as u64;
    if baud == 0 || (clock as u64) < denom {
        return None;
    }
    let mut integer = clock as u64 / denom;
    let rem = clock as u64 % denom;
    let mut fraction = (rem * 64 + denom / 2) / denom;
    if fraction == 64 {
        integer += 1;
        fraction = 0;
    }
    (integer <= 0xFFFF).then_some((integer as u32, fraction as u32))
}

/// 16550: `clock / (16 * baud)` rounded to nearest; zero is not a divisor.
pub(crate) fn ns16550(clock: u32, baud: u32) -> Option<u16> {
    let denom = 16 * baud as u64;
    if baud == 0 {
        return None;
    }
    let divisor = (clock as u64 + denom / 2) / denom;
    u16::try_from(divisor).ok().filter(|&d| d != 0)
}

/// Mini-UART: the baud register holds `clock / (8 * baud) - 1`.
pub(crate) fn mini_uart(clock: u32, baud: u32) -> Option<u16> {
    let denom = 8 * baud as u64;
    if baud == 0 {
        return None;
    }
    let divisor = (clock as u64 + denom / 2) / denom;
    u16::try_from(divisor.checked_sub(1)?).ok()
}
