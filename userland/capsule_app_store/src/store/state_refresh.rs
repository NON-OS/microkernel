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

//! Fetching the catalogue.

use super::market;
use super::state::State;

impl State {
    /// Ask the market for the catalogue again.
    pub fn refresh(&mut self) {
        self.loaded = true;
        (self.memory, self.room) = super::machine::read();
        (self.route, self.fetch) = (crate::net::Route::for_installs(), None);
        let port = market::port();
        self.port = port;
        if port == 0 {
            self.listings.clear();
            self.trouble =
                Some(b"the market service is not running yet\nasking again; r asks now".to_vec());
            return;
        }
        match market::list_apps(port, market::next_id()) {
            Ok(found) => {
                self.listings = found;
                self.trouble = None;
                // One installed earlier in this session reads as installed.
                self.poll_all();
            }
            /*
             * The call failed, which is not the same as the catalogue being
             * empty and must not be reported as it; what the market answered,
             * if it answered, says which failure it was.
             */
            Err(why) => {
                self.listings.clear();
                self.trouble = Some(why.catalogue_trouble());
            }
        }
        self.cursor = 0;
        self.scroll = 0;
    }

    /// Whether the last ask found no market service to ask: one that has
    /// not registered yet, as early in a boot. A market that answered, even
    /// with a refusal, is not asked again unbidden.
    pub fn market_not_up(&self) -> bool {
        self.port == 0 && self.listings.is_empty()
    }
}
