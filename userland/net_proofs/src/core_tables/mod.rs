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

//! net.core's tables of its clients' TCP connections and UDP ports, from
//! capsule source. Both are generic over the stack's socket handle, so the
//! proofs hold them with a plain number in its place. The real code's own
//! style is allowed on the include rather than restyled here.

#[allow(clippy::new_without_default)]
#[path = "../../../capsule_net_core/src/handles/table.rs"]
pub mod handles;
#[allow(clippy::new_without_default, clippy::unnecessary_map_or)]
#[path = "../../../capsule_net_core/src/udp_ports/table.rs"]
pub mod ports;
