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
//! What a port carries, and the swing row, rate and lanes it means.

use nonos_intel_display_hw::error::Error;

use super::swing_hdmi::{dp_row, HDMI};
use super::swing_table::{Swing, DP_HBR, EDP_HBR2};

/// A port's signal: DisplayPort or eDP at a link rate in Mbit/s per lane,
/// over 1, 2 or 4 lanes, at the sink's swing and pre-emphasis levels; or
/// HDMI at one of the table's seven rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signal {
    Dp { mbps: u32, lanes: u32, vs: u32, pe: u32 },
    Edp { mbps: u32, lanes: u32, vs: u32, pe: u32 },
    Hdmi { row: usize, symbol_khz: u32 },
}

/// The DisplayPort link rates the combo PLL table lists (Vol 12 p. 178).
const RATES: [u32; 8] = [1620, 2160, 2700, 3240, 4320, 5400, 6480, 8100];

/// The row, the bit rate per lane in Mbit/s, the lane count, and whether
/// the signal is DisplayPort (for cmnkeeper). DP is offered up to 2.7
/// Gbit/s and eDP up to 5.4 (see `swing_table`); HDMI's bit rate is ten
/// times its symbol clock, over four lanes.
pub fn pick(sig: Signal) -> Result<(Swing, u32, u32, bool), Error> {
    let found = match sig {
        Signal::Dp { mbps, lanes, vs, pe } if RATES.contains(&mbps) && mbps <= 2700 => {
            dp_row(vs, pe).map(|r| (DP_HBR[r], mbps, lanes, true))
        }
        Signal::Edp { mbps, lanes, vs, pe } if RATES.contains(&mbps) && mbps <= 5400 => {
            dp_row(vs, pe).map(|r| (EDP_HBR2[r], mbps, lanes, true))
        }
        // HDMI symbol clocks run 20 to 600 MHz (Vol 12 p. 170).
        Signal::Hdmi { row, symbol_khz } if (20_000..=600_000).contains(&symbol_khz) => {
            HDMI.get(row).map(|s| (*s, symbol_khz * 10 / 1000, 4, false))
        }
        _ => None,
    };
    found.ok_or(Error::Argument)
}
