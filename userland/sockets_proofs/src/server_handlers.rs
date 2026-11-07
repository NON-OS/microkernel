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

//! net.sockets' mixnet frame and the remainders a short read leaves, from
//! the capsule's own source. The read that calls the transport stays out.

#[path = "../../capsule_net_sockets/src/server/handlers/listen_route.rs"]
pub mod listen_route;
#[path = "../../capsule_net_sockets/src/server/handlers/listen_who.rs"]
pub mod listen_who;
#[path = "../../capsule_net_sockets/src/server/handlers/mixnet_frame/mod.rs"]
pub mod mixnet_frame;
#[path = "../../capsule_net_sockets/src/server/handlers/mixnet_residual/mod.rs"]
pub mod mixnet_residual;
#[path = "../../capsule_net_sockets/src/server/handlers/passive.rs"]
pub mod passive;
