// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! One disk on the block wire: an endpoint and the unit behind it.

use super::endpoint::{Endpoint, MAX_UNITS};

#[derive(Clone, Copy)]
pub(super) struct Target {
    pub e: &'static Endpoint,
    pub unit: u16,
}

impl Target {
    /// The unit's slot in the endpoint's per-unit state.
    pub(super) fn at(&self) -> usize {
        self.unit as usize
    }
}

/// The units of `e` the client asks: as many as it declares, and no more
/// than it keeps state for.
pub(super) fn units(e: &Endpoint) -> u16 {
    e.units.min(MAX_UNITS as u16)
}
