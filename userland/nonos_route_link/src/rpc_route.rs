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

//! A chain RPC's route in a `devnet` build (P5): the wallet's, except for the
//! local test chain's one name, which is dialled directly. A release build
//! has no such exception, so it does not compile this at all.

use crate::pick::Route;

impl Route {
    /// `self`, the wallet's route, for `host`; Direct for the local chain's RPC.
    pub fn for_rpc(self, host: &str) -> Route {
        if crate::devnet::is_devnet_rpc(host) {
            Route::Direct
        } else {
            self
        }
    }
}
