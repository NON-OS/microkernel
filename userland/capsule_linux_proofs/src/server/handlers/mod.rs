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

#[path = "../../../../capsule_net_sockets/src/server/handlers/io.rs"]
pub mod io;

#[path = "../../../../capsule_net_sockets/src/server/handlers/connect/parse_host.rs"]
pub mod parse_host;

/* How net.sockets decides where a connect by name goes, and the address
 * reader it uses first: a mixnet socket is refused a name, never resolved. */
#[path = "../../../../capsule_net_sockets/src/server/handlers/connect/host_target.rs"]
pub mod host_target;
#[path = "../../../../capsule_net_sockets/src/server/handlers/connect/parse_ipv4.rs"]
pub mod parse_ipv4;
