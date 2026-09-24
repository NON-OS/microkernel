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
        let port = market::port();
        if port == 0 {
            self.listings.clear();
            self.trouble = Some(b"market service has not announced itself");
            return;
        }
        match market::list_apps(port, market::next_id()) {
            Some(found) => {
                self.listings = found;
                self.trouble = None;
            }
            /*
             * The call failed, which is not the same as the catalogue being
             * empty and must not be reported as it.
             */
            None => {
                self.listings.clear();
                self.trouble = Some(b"market did not answer");
            }
        }
        self.cursor = 0;
        self.scroll = 0;
        self.select();
    }
}
