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

//! Link training (DisplayPort 1.4a section 3.5.1.2): clock recovery with
//! TPS1, then channel equalisation with TPS2 or TPS3, at the fastest rate
//! and widest link both ends allow, stepping down the rate and then the
//! lane count when a phase fails.

use super::caps::Caps;
use super::dpcd::{rate_code, LINK_BW_SET, MAIN_LINK_CHANNEL_CODING_SET, TRAINING_PATTERN_SET};
use super::lane::Drive;
pub use super::link::{Link, Pattern, Source, TrainError, Trained};
use super::native::write;
use super::{train_cr, train_eq};

const RATES: [u32; 4] = [8100, 5400, 2700, 1620];

/// Train at the best rate and width both ends allow (`max` the source's).
pub fn train(s: &mut dyn Source, caps: &Caps, max: (u32, u32)) -> Result<Trained, TrainError> {
    let lanes_top = caps.max_lanes.min(max.1);
    for lanes in [4, 2, 1].into_iter().filter(|&l| l <= lanes_top) {
        for mbps in RATES.into_iter().filter(|&r| r <= caps.max_rate_mbps.min(max.0)) {
            match attempt(s, caps, mbps, lanes) {
                Ok(drive) => return Ok(Trained { mbps, lanes, drive }),
                Err(TrainError::Failed) => s.stop(),
                Err(e) => {
                    s.stop();
                    return Err(e);
                }
            }
        }
    }
    Err(TrainError::Failed)
}

fn attempt(s: &mut dyn Source, caps: &Caps, mbps: u32, lanes: u32) -> Result<Drive, TrainError> {
    s.start(mbps, lanes)?;
    let code = rate_code(mbps).ok_or(TrainError::Transmitter)?;
    // LINK_BW_SET and LANE_COUNT_SET (0x101, enhanced framing in bit 7).
    write(s, LINK_BW_SET, &[code, lanes as u8 | ((caps.enhanced_framing as u8) << 7)])?;
    if caps.revision >= 0x12 {
        // 8b/10b; the field exists from DisplayPort 1.2.
        write(s, MAIN_LINK_CHANNEL_CODING_SET, &[1])?;
    }
    let drive = train_cr::clock_recovery(s, lanes)?;
    let drive = train_eq::equalise(s, caps, lanes, drive)?;
    s.pattern(Pattern::Off);
    write(s, TRAINING_PATTERN_SET, &[0])?;
    Ok(drive)
}
