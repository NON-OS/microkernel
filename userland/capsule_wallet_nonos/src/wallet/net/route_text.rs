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

//! Which network the wallet's requests went over, as the screens say it.
//!
//! The status line used to say "Nym" whenever net.nym answered a health
//! check, while every request went straight to the RPC host. It now says
//! the route the last check actually took, and nothing before the first.

use super::wallet_route::WalletRoute;

/// The word on the status line, or None before anything was checked.
pub fn route_part(route: Option<WalletRoute>) -> Option<&'static str> {
    route.map(WalletRoute::label)
}

/// The Route row on the network card: the RPC host and how it is reached.
pub fn route_value(route: Option<WalletRoute>) -> &'static str {
    match route {
        Some(WalletRoute::Nym(_)) => "public RPC via Nym",
        Some(WalletRoute::Anon(_)) => "public RPC via Anyone",
        Some(WalletRoute::Down(_)) => "no route",
        None => "public RPC",
    }
}
