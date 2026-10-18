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

//! The control queue's named state, and the count of stalls
//! recovered in place that `server::recover` keeps.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Health {
    /// Commands reach the device. It may have stalled and been recovered in
    /// place since; the driver counts those.
    Live,
    /// The device stalled and no recovery try brought it back. It is left
    /// reset and every surface stays mapped (the host may hold none, but
    /// nothing is handed back on a guess); every output answers LOST and
    /// dark, FLUSH and MODE_SET ENXIO, and the compositor was told LEAVING.
    Down,
}

/// Stalls recovered in place, recoveries in a row, and the queue's count of
/// commands published and the clock when the last recovery ended.
#[derive(Clone, Copy, Debug, Default)]
pub struct Tally {
    pub recoveries: u32,
    pub in_a_row: u32,
    pub recovered_at: u16,
    pub recovered_ms: u64,
}
