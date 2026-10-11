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

//! Every period the engine has finished since the last look is written
//! again from the queue, silence where the queue runs short. The period the
//! engine plays is never touched; three are always ahead of it, about 130
//! ms of audio, as with the HD Audio driver.

use super::stream::Stream;
use crate::audio::Period;
use crate::constants::ring::{N_PERIODS, PERIOD_BYTES};
use crate::setup::Driver;
use crate::{engine, say};

pub fn refill(d: &mut Driver, st: &mut Stream) {
    let playing = engine::follow(&d.bus) as usize % N_PERIODS;
    while st.wpos != playing {
        let filled = st.q.pop_into(d.period(st.wpos));
        match st.periods.period(filled, PERIOD_BYTES) {
            Period::Played if st.periods.played == 1 => say::first_period(),
            Period::Underrun if st.periods.underruns == 1 => say::first_underrun(),
            _ => {}
        }
        st.wpos = (st.wpos + 1) % N_PERIODS;
    }
}
