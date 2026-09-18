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

//! The figure as the chain counts it.

use super::Amount;

impl Amount {
    /// The typed digits shifted up by whatever precision the fraction did not
    /// already use.
    ///
    /// Saturating, never wrapping. An amount past `u128` is not one this wallet
    /// can sign, and wrapping it would turn a refusal into a different, payable
    /// figure.
    pub fn scaled(&self, decimals: u32) -> u128 {
        let places = self.places.min(decimals);
        let mut v = self.value;
        let mut shift = decimals - places;
        while shift > 0 {
            v = v.saturating_mul(10);
            shift -= 1;
        }
        v
    }
}
