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

//! The HDMI DCO and dividers as DPLL_CFGCR1 and DPLL_CFGCR2 hold them (SKL
//! PRM Vol 2c): the DCO as a multiple of the 24 MHz reference, a 9-bit
//! whole part and a 15-bit fraction. Field encodings unverified.

use super::dpll_hdmi::Dco;
use super::dpll_split::split;

pub const REF_HZ: u64 = 24_000_000;

/// DPLL_CFGCR1 and DPLL_CFGCR2 for `d`.
pub fn cfgcr(d: &Dco) -> Option<(u32, u32)> {
    let (p0, p1, p2) = split(d.divider)?;
    let whole = d.dco_hz / REF_HZ;
    let frac = (d.dco_hz % REF_HZ) * (1 << 15) / REF_HZ;
    if whole >= 1 << 9 || p1 > 255 {
        return None;
    }
    // CFGCR1: frequency enable 31, DCO fraction 23:9, DCO integer 8:0.
    let c1 = (1 << 31) | ((frac as u32) << 9) | whole as u32;
    let pdiv = match p0 {
        1 => 0,
        2 => 1,
        3 => 2,
        _ => 4,
    };
    let kdiv = match p2 {
        5 => 0,
        2 => 1,
        3 => 2,
        _ => 3,
    };
    let central = match d.central_mhz {
        9600 => 0,
        9000 => 1,
        _ => 3,
    };
    // CFGCR2: qdiv ratio 15:8, qdiv mode 7, kdiv 6:5, pdiv 4:2, centre 1:0.
    let c2 = (p1 << 8) | (((p1 != 1) as u32) << 7) | (kdiv << 5) | (pdiv << 2) | central;
    Some((c1, c2))
}
