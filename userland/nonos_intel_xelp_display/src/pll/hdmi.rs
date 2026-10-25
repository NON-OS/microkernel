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
//! HDMI and DVI on the combo PHY: the dividers for a symbol clock, found
//! the way Vol 12 describes (p. 180 to 182). The PLL's output is five
//! times the symbol clock; for each divider Vol 12 lists, the DCO is that
//! output times the divider, and the DCO inside 7,998 to 10,000 MHz that
//! lies nearest 8,999 MHz wins, the first listed on a tie.

use super::dividers::Dividers;
use super::split::split;
use nonos_intel_display_hw::error::Error;

/// The dividers Vol 12 lists, in its order (p. 181).
pub const LIST: [u32; 46] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 24, 28, 30, 32, 36, 40, 42, 44, 48, 50, 52, 54, 56, 60, 64,
    66, 68, 70, 72, 76, 78, 80, 84, 88, 90, 92, 96, 98, 100, 102, 3, 5, 7, 9, 15, 21,
];
const DCO_MIN_KHZ: u64 = 7_998_000;
const DCO_MAX_KHZ: u64 = 10_000_000;
const DCO_MID_KHZ: u64 = 8_999_000;

/// The dividers for `symbol_khz` on a reference of `ref_khz`. Refused: a
/// symbol clock outside 20 to 600 MHz or strictly between 500 and 533.2
/// MHz (p. 170), a reference DSSM does not name, and a clock no divider
/// reaches. A 38.4 MHz reference counts as 19.2, which the DPLL divides it
/// to itself (p. 180).
pub fn dividers(symbol_khz: u32, ref_khz: u32) -> Result<Dividers, Error> {
    let s = symbol_khz as u64;
    let gap = s > 500_000 && s < 533_200;
    if !(20_000..=600_000).contains(&s) || gap {
        return Err(Error::Argument);
    }
    let r = match ref_khz {
        24_000 => 24_000u64,
        19_200 | 38_400 => 19_200,
        _ => return Err(Error::Argument),
    };
    let afe = 5 * s;
    let fits = |d: &&u32| (DCO_MIN_KHZ..=DCO_MAX_KHZ).contains(&(afe * **d as u64));
    let mut best: Option<u32> = None;
    for d in LIST.iter().filter(fits) {
        let off = |d: u32| (afe * d as u64).abs_diff(DCO_MID_KHZ);
        if best.is_none_or(|b| off(*d) < off(b)) {
            best = Some(*d);
        }
    }
    let div = best.ok_or(Error::Argument)?;
    let (p, q, k) = split(div);
    let dco = afe * div as u64;
    let integer = (dco / r) as u32;
    let fraction = (((dco % r) << 15) / r) as u32;
    Ok(Dividers { integer, fraction, p, q, k })
}
