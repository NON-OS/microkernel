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

//! Whether a trained link still holds (DisplayPort 1.4a, link
//! maintenance; section number unverified): the source reads the lane
//! status after a sink's IRQ_HPD and retrains when a lane lost clock
//! recovery, equalisation or symbol lock, or interlane alignment. The capsule also asks on its hot-plug
//! poll, since NONOS drives the display without interrupts.

use super::aux::{Aux, AuxError};
use super::dpcd::LANE0_1_STATUS;
use super::native;
use super::status::LaneStatus;

/// True when the `lanes` the link was trained on no longer all hold;
/// the link must then be trained again before the mode shows.
pub fn link_lost(aux: &mut dyn Aux, lanes: u32) -> Result<bool, AuxError> {
    let mut st = LaneStatus::default();
    native::read(aux, LANE0_1_STATUS, &mut st.0)?;
    Ok(!st.eq_done(lanes))
}
