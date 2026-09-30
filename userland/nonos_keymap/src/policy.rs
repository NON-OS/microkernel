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

//! The policy store's keyboard layout field, as the drivers resolve it.
//!
//! The field indexes the policy's label list (US QWERTY, US Dvorak, UK,
//! German, French AZERTY, Italian, Spanish, Russian, Japanese, Chinese). Only
//! the rows with tables here take effect; the rest map to nothing, so a driver
//! keeps the layout it has rather than pretend to type Dvorak.

use crate::layout::Layout;

/// Policy indices of the layouts the drivers can resolve, in the policy's
/// order. Setup offers exactly these.
pub const POLICY_LAYOUTS: [u8; 6] = [0, 2, 3, 4, 5, 6];

impl Layout {
    /// The layout a policy index names, or `None` when no table exists for it.
    pub fn from_policy(index: u8) -> Option<Layout> {
        Some(match index {
            0 => Layout::Us,
            2 => Layout::Uk,
            3 => Layout::De,
            4 => Layout::Fr,
            5 => Layout::It,
            6 => Layout::Es,
            _ => return None,
        })
    }
}
