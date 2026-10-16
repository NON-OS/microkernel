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

//! Where the join stands, why it failed, and what it chose.

use super::*;

impl Mlme {
    pub fn state(&self) -> MlmeState {
        self.state
    }

    /// Why the join failed, once `Failed`.
    pub fn failure(&self) -> Option<MlmeFailure> {
        self.failure
    }

    /// The security chosen from the beacon, once a BSS was selected.
    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }
}
