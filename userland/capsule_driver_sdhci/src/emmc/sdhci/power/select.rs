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

//! The first supply voltage and the one the card and host share.

use super::ocr::OCR_LOW_RESERVED;
use super::{vdd_for_bit, Vdd};

/// The voltage the bus is first powered at: the highest the host offers
/// (mmc_power_up takes fls(ocr_avail)).
pub fn first_vdd(host: u32) -> Option<Vdd> {
    if host == 0 {
        return None;
    }
    vdd_for_bit(31 - host.leading_zeros())
}

/// mmc_select_voltage: the lowest voltage in both `card` and `host`, as the
/// OCR window CMD1 then carries (two adjacent bits from the lowest common
/// one, kept only where both agree) and the supply it names.
pub fn select(card: u32, host: u32) -> Option<(u32, Vdd)> {
    let common = card & !OCR_LOW_RESERVED & host;
    if common == 0 {
        return None;
    }
    let bit = common.trailing_zeros();
    let window = common & (3 << bit);
    vdd_for_bit(bit).map(|v| (window, v))
}
