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

//! What selecting a listing costs: nothing. The selection only moves the
//! cursor; what the market says about a listing is asked on a tick, once,
//! and kept on the listing (`fill`).

use super::listing::Listing;
use super::state::State;

impl State {
    pub fn current(&self) -> Option<&Listing> {
        self.listings.get(self.current_index()?)
    }

    pub fn current_mut(&mut self) -> Option<&mut Listing> {
        let at = self.current_index()?;
        self.listings.get_mut(at)
    }

    /// The selected listing's index into `listings`.
    pub fn current_index(&self) -> Option<usize> {
        self.visible().get(self.cursor).copied()
    }
}
