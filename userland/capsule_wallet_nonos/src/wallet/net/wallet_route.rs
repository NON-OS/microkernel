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

//! The routes the wallet holds: Nym, Anyone or none, and no Direct to hold.

use super::private_route::WAITING;
use super::Route;

/* Every route a wallet field, argument or status carries. Route link's
 * Direct has no case here, so no wallet code can take or show one. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WalletRoute {
    /* Through net.socks5 or net.anon, at this port. */
    Nym(u32),
    Anon(u32),
    /// No route the wallet may take, and why.
    Down(&'static str),
}

impl WalletRoute {
    /// The one way in from route link. A Direct handed in by mistake is no
    /// route: it opens nothing and the wallet waits for Nym or Anyone.
    pub fn from_link(route: Route) -> WalletRoute {
        match route {
            Route::Nym(port) => WalletRoute::Nym(port),
            Route::Anon(port) => WalletRoute::Anon(port),
            Route::Down(why) => WalletRoute::Down(why),
            Route::Direct => WalletRoute::Down(WAITING),
        }
    }

    /// The route link's stream APIs take, or None when there is no route.
    pub fn link(self) -> Option<Route> {
        match self {
            WalletRoute::Down(_) => None,
            other => Some(other.described()),
        }
    }

    /// The route in a sentence, as route link words it (`described`).
    pub fn name(self) -> &'static str {
        self.described().name()
    }

    /// A word for a status line, as route link words it.
    pub fn label(self) -> &'static str {
        self.described().label()
    }

    /// How long a read waits for the far end, as route link sets it.
    pub fn patience_ms(self) -> u64 {
        self.described().patience_ms()
    }

    fn described(self) -> Route {
        match self {
            WalletRoute::Nym(port) => Route::Nym(port),
            WalletRoute::Anon(port) => Route::Anon(port),
            WalletRoute::Down(why) => Route::Down(why),
        }
    }
}
