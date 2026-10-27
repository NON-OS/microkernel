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

//! The network a guest's stream leaves through follows the person's choice.
//!
//! A guest's stream on Nym went to net.sockets' mixnet socket, whose frames
//! carry an address, so every name a guest looked up was refused and the
//! Monero wallet reached no node. Held here over every default the policy
//! store could hold, or none, with each network up or down, through
//! nonos_route_link's own pick: Anyone goes to net.anon, Nym and Direct go
//! by name to net.socks5, and a default that cannot be read, or a network
//! whose service has not registered, is waited for and never becomes Nym.
//! With it, the SOCKS endpoint (socks/) and a stream that did not open:
//! the retry rule, its errno and the line a person reads (routed/).

mod by_name;
mod pick;
mod routed;
mod socks;
mod table;
