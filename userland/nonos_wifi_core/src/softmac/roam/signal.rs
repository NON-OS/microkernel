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

//! How well the session's AP is heard: a running average of its beacons'
//! signal, an eighth of each new beacon moving it. One weak beacon does not
//! start a look; a run of them does.

/// Below this average a look for a better AP of the same network starts.
pub const WEAK_DBM: i16 = -75;
/// Beacons heard before the average counts.
pub const SAMPLES_MIN: u8 = 8;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Signal {
    /// The average in eighths of a dBm.
    avg_x8: i32,
    samples: u8,
}

impl Signal {
    pub fn heard(&mut self, rssi_dbm: i8) {
        let x8 = i32::from(rssi_dbm) * 8;
        self.avg_x8 = if self.samples == 0 { x8 } else { self.avg_x8 + (x8 - self.avg_x8) / 8 };
        self.samples = self.samples.saturating_add(1);
    }

    /// The average in dBm, rounded toward zero.
    pub fn avg_dbm(&self) -> i16 {
        (self.avg_x8 / 8) as i16
    }

    pub fn weak(&self) -> bool {
        self.samples >= SAMPLES_MIN && self.avg_dbm() < WEAK_DBM
    }

    /// Start over, as on a new AP.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
