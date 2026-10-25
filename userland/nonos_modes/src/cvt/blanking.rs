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

//! The one entry point to CVT 1.2, by blanking.

use super::{reduced, reduced_v2, standard};
use nonos_edid::Detailed;

/// Which CVT 1.2 blanking a computed timing uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blanking {
    /// For CRTs and anything that asks for it; the most blanking.
    Standard,
    /// Reduced blanking version 1, the usual one for digital panels.
    Reduced,
    /// Reduced blanking version 2 (CVT 1.2), the least blanking.
    ReducedV2,
}

/// A progressive CVT 1.2 timing; `refresh_mhz` in millihertz. None for
/// a size or rate the formulas cannot meet.
pub fn cvt(width: u32, height: u32, refresh_mhz: u32, b: Blanking) -> Option<Detailed> {
    match b {
        Blanking::Standard => standard(width, height, refresh_mhz),
        Blanking::Reduced => reduced(width, height, refresh_mhz),
        Blanking::ReducedV2 => reduced_v2(width, height, refresh_mhz),
    }
}
