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

/*
 * Making a kept name or tier from what setup holds, held to the same rules
 * a record read back is.
 */

use super::kept::{Name, Tier};
use super::rules::{name_ok, tier_ok};

impl Name {
    /* `None` for a name setup's name step would not take. */
    pub fn new(s: &[u8]) -> Option<Self> {
        Self::from_ok(s, name_ok)
    }
}

impl Tier {
    /* `None` for anything that is not a tier's name. */
    pub fn new(s: &[u8]) -> Option<Self> {
        Self::from_ok(s, tier_ok)
    }
}
