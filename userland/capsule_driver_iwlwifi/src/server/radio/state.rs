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

//! The radio's state, and the rest between sweeps.

use nonos_wifi_core::scan_list::ScanResults;

use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::sweep::Sweep;

use super::bring::{Seen, Up};
use super::join::Join;

/// The rest between one sweep's end and the next sweep's start.
pub(super) const REST_MS: u64 = 3000;
/// Stalled sweeps reported on the console; later ones are only counted.
pub(super) const STALLS_SAID: u32 = 3;

pub(super) enum State {
    Up(Up),
    Down(Failure),
}

pub struct Radio {
    pub(super) seen: Seen,
    pub(super) state: State,
    pub(super) sweep: Sweep,
    /// The networks heard, aged by sweep.
    pub results: ScanResults,
    /// Beacons and probe responses that parsed.
    pub beacons: u32,
    pub(super) next_ms: u64,
    pub(super) join: Join,
}
