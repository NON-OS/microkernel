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
 * What a route is called where a person reads it. How long a reader on it
 * waits for the far end is in patience.rs.
 */

use crate::pick::Route;

impl Route {
    /* The route in a sentence: "fetched through the Nym mixnet". A route
     * that is down is its reason. */
    pub fn name(self) -> &'static str {
        match self {
            Route::Nym(_) => "through the Nym mixnet",
            Route::Anon(_) => "through the Anyone onion network",
            Route::Direct => "over a direct connection",
            Route::Down(why) => why,
        }
    }

    /* A word for a status line. */
    pub fn label(self) -> &'static str {
        match self {
            Route::Nym(_) => "Nym",
            Route::Anon(_) => "Anyone",
            Route::Direct => "Direct",
            Route::Down(_) => "no route",
        }
    }

    pub fn is_anonymous(self) -> bool {
        matches!(self, Route::Nym(_) | Route::Anon(_))
    }
}
