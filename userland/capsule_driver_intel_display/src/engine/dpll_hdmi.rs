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

//! The HDMI clock on a Skylake DPLL, as the PRM's HDMI mode programming
//! steps give it (SKL PRM Vol 12, DPLL HDMI mode): the AFE clock is five
//! times the pixel clock; a DCO near one of three central frequencies is
//! a whole multiple of it, within 1% above and 6% below the centre; an
//! even multiple is preferred to an odd one, and then the one nearest
//! its centre.

/// The central frequencies, MHz.
pub const CENTRAL_MHZ: [u64; 3] = [8400, 9000, 9600];
const EVEN: [u64; 36] = [
    4, 6, 8, 10, 12, 14, 16, 18, 20, 24, 28, 30, 32, 36, 40, 42, 44, 48, 52, 54, 56, 60, 64, 66,
    68, 70, 72, 76, 78, 80, 84, 88, 90, 92, 96, 98,
];
const ODD: [u64; 7] = [3, 5, 7, 9, 15, 21, 35];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dco {
    /// The whole divider from the DCO to the AFE clock.
    pub divider: u32,
    pub central_mhz: u32,
    pub dco_hz: u64,
}

/// The DCO for `pixel_khz`, None when no divider lands in range.
pub fn dco(pixel_khz: u32) -> Option<Dco> {
    let afe_hz = pixel_khz as u64 * 5000;
    if afe_hz == 0 {
        return None;
    }
    pick(afe_hz, &EVEN).or_else(|| pick(afe_hz, &ODD))
}

fn pick(afe_hz: u64, dividers: &[u64]) -> Option<Dco> {
    let mut best: Option<(u64, Dco)> = None;
    for &p in dividers {
        for c in CENTRAL_MHZ {
            let dco_hz = p * afe_hz;
            let centre = c * 1_000_000;
            // Deviation in units of 0.01% of the centre.
            let over = dco_hz >= centre;
            // u128: a clock at u32's top runs the product past u64.
            let dev = (dco_hz.abs_diff(centre) as u128 * 10_000 / centre as u128) as u64;
            let ok = if over { dev <= 100 } else { dev <= 600 };
            if ok && best.is_none_or(|(d, _)| dev < d) {
                best = Some((dev, Dco { divider: p as u32, central_mhz: c as u32, dco_hz }));
            }
        }
    }
    best.map(|(_, d)| d)
}
