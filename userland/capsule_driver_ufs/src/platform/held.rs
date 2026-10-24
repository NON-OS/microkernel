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

//! Everything one bring-up holds from the broker, in the order it goes back.

use super::grants::{Claim, DmaRegion};

/// Field order is drop order: the regions go back before the claim, which
/// is held for its drop alone.
pub struct Held {
    pub regions: [DmaRegion; 4],
    pub(super) _claim: Claim,
}

impl Held {
    /// Release the claim alone, which stops bus mastering, and never unmap
    /// the regions: a host still running may yet write them.
    pub fn leak_regions(self) {
        let Held { regions, _claim } = self;
        core::mem::forget(regions);
    }
}
