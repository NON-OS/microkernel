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

//! Clock recovery (DisplayPort 1.4a section 3.5.1.2.1): TPS1 unscrambled,
//! 100 us between tries, the drive the sink asks for applied each time.
//! It fails when the same swing has been tried five times, or when the
//! sink still has no lock at the highest swing.

use super::aux::Aux;
use super::dpcd::{LANE0_1_STATUS, TRAINING_PATTERN_SET};
use super::lane::{Drive, LaneStatus};
use super::native::{read, write};
use super::train::{Pattern, Source, TrainError};

const CR_WAIT_US: u32 = 100;
const SAME_SWING_TRIES: u32 = 5;
/// Every pass at most, whatever the sink asks: a sink that flips between
/// two swings resets the count above and would otherwise never end. Our
/// bound, not a figure from the standard.
pub const CR_TRIES: u32 = 10;

/// TRAINING_PATTERN_SET and every lane's TRAINING_LANEx_SET in one write.
pub(super) fn set(aux: &mut dyn Aux, pattern: u8, d: Drive, lanes: u32) -> Result<(), TrainError> {
    let mut b = [pattern, 0, 0, 0, 0];
    b[1..=lanes as usize].fill(d.lane_set());
    write(aux, TRAINING_PATTERN_SET, &b[..=lanes as usize])?;
    Ok(())
}

pub(super) fn status(aux: &mut dyn Aux) -> Result<LaneStatus, TrainError> {
    let mut s = LaneStatus::default();
    read(aux, LANE0_1_STATUS, &mut s.0)?;
    Ok(s)
}

pub fn clock_recovery(s: &mut dyn Source, lanes: u32) -> Result<Drive, TrainError> {
    let mut d = Drive::default();
    s.pattern(Pattern::Tps1);
    s.drive(d);
    // 0x21: TPS1 with scrambling disabled.
    set(s, 0x21, d, lanes)?;
    let mut same = 1;
    for _ in 0..CR_TRIES {
        s.wait_us(CR_WAIT_US);
        let st = status(s)?;
        if st.cr_done(lanes) {
            return Ok(d);
        }
        let want = st.requested(lanes);
        if d.swing == 3 || (want.swing == d.swing && same >= SAME_SWING_TRIES) {
            return Err(TrainError::Failed);
        }
        same = if want.swing == d.swing { same + 1 } else { 1 };
        d = want;
        s.drive(d);
        set(s, 0x21, d, lanes)?;
    }
    Err(TrainError::Failed)
}
