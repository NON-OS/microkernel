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

//! A guest's connect by name on a mixnet socket never reaches net.dns.
//!
//! The guest's stream goes through the mixnet, but net.sockets used to
//! resolve the name it was handed through net.dns first, in the clear, for
//! every socket kind, so every host a guest program reached was named to
//! the resolver and the path. The name now travels nowhere: the mixnet's
//! frames carry an address, not a name, so the connect is refused and the
//! guest is told the network is unreachable. Held end to end here: the
//! capsule's own encoder, net.sockets' own reader, and its decision.

mod direct;
mod helpers;
mod refused;
mod status;
