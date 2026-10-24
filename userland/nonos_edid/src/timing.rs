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

use crate::base::dtd::Detailed;
use crate::source::Source;

/// One mode the monitor says it takes. Sources other than the detailed
/// ones name only the picture and the rate; `detail` is None for those
/// and a driver computes the timing (DMT or CVT) itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timing {
    pub width: u32,
    pub height: u32,
    /// Frames per second, fields for an interlaced timing, in millihertz.
    pub refresh_mhz: u32,
    pub interlaced: bool,
    /// The source asked for reduced blanking (CVT RB).
    pub reduced_blanking: bool,
    /// The monitor's preferred mode: the base block's first detailed
    /// timing, or a DisplayID timing with its preferred bit.
    pub preferred: bool,
    /// A CTA-861 short video descriptor marked native.
    pub native: bool,
    pub source: Source,
    pub detail: Option<Detailed>,
}

impl Timing {
    /// A timing named only by its picture and nominal rate.
    pub fn nominal(width: u32, height: u32, hz: u32, source: Source) -> Self {
        Timing {
            width,
            height,
            refresh_mhz: hz.saturating_mul(1000),
            interlaced: false,
            reduced_blanking: false,
            preferred: false,
            native: false,
            source,
            detail: None,
        }
    }

    /// A timing carrying its full detail.
    pub fn detailed(d: Detailed, source: Source) -> Self {
        Timing {
            interlaced: d.interlaced,
            refresh_mhz: d.refresh_mhz(),
            detail: Some(d),
            ..Timing::nominal(d.width(), d.height(), 0, source)
        }
    }
}
