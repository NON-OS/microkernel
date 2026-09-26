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

//! What selecting a listing costs.

use super::market;
use super::state::State;

impl State {
    /// Ask the market why the selected listing can or cannot be installed.
    pub fn select(&mut self) {
        self.ready = None;
        self.detail = None;
        let Some(listing) = self.current() else { return };
        let (id, port) = (listing.id.clone(), market::port());
        self.detail = market::get_app(port, market::next_id(), &id);
        /*
         * The release is left unnamed because the capsule resolves the default
         * when it is.
         */
        self.ready = market::install_ready(port, market::next_id(), &id, &[]);
    }

    pub fn current(&self) -> Option<&super::listing::Listing> {
        self.listings.get(*self.visible().get(self.cursor)?)
    }
}
