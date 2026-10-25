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

//! The wallet's route for the next connection, asked of this boot now.

use nonos_route_link::Route;

use super::wallet_route::WalletRoute;

impl WalletRoute {
    /// Route link's `for_wallet` (Nym, else Anyone, else no route), held in
    /// the wallet's own type, which has no Direct.
    pub fn for_wallet() -> WalletRoute {
        WalletRoute::from_link(Route::for_wallet())
    }
}
