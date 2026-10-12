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

//! The first supply, and the one the card and host share.

use super::ocr::{OCR_165_195, OCR_29_31, OCR_32_34, OCR_LOW_RESERVED};
use super::{vdd_for_bit, Vdd};

/// The highest voltage the host offers.
pub fn first_vdd(host: u32) -> Option<Vdd> {
    let top = host.checked_ilog2()?;
    vdd_for_bit(top)
}

/// The lowest voltage both `card` and `host` name, and the OCR window CMD1
/// carries for it: the shared bits that name that supply and no other.
pub fn select(card: u32, host: u32) -> Option<(u32, Vdd)> {
    let shared = card & host & !OCR_LOW_RESERVED;
    if shared == 0 {
        return None;
    }
    let vdd = vdd_for_bit(shared.trailing_zeros())?;
    Some((shared & bits_of(vdd), vdd))
}

const fn bits_of(v: Vdd) -> u32 {
    match v {
        Vdd::V18 => OCR_165_195,
        Vdd::V30 => OCR_29_31,
        Vdd::V33 => OCR_32_34,
    }
}
