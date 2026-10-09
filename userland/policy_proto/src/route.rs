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

//! The network the system's own traffic leaves through by default: the
//! browser's first choice and the one the model fetcher, the Terminal and the
//! wallet take through nonos_route_link. Package installs by the Linux
//! personality do not read it: they always cross the mixnet, except to a
//! mirror on a private address. Each is a choice, never a fallback: a route that cannot carry a
//! request fails it rather than reverting to another, which would leak
//! exactly when the network is worst.

/// The Nym mixnet: who talks to whom is hidden even from an observer of the
/// whole network, at the cost of delay. The default.
pub const NYM: u8 = 0;
/// The Anyone onion network: three relays between this machine and the site,
/// faster than the mixnet.
pub const ANYONE: u8 = 1;
/// No anonymity network: fastest, and every site and the local network see
/// this machine's address.
pub const DIRECT: u8 = 2;

pub const ROUTE_LABELS: &[&[u8]] = &[b"Nym mixnet", b"Anyone network", b"Direct"];

/// Whether `route` is one this build knows.
pub fn known(route: u8) -> bool {
    (route as usize) < ROUTE_LABELS.len()
}
