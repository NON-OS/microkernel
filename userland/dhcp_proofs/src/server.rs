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

//! net.dhcp.client's `server` module as far as a frame goes before dispatch: the
//! header decode, and the reply the loop answers a refused frame with. The
//! handlers and the receive loop stay out.

#[path = "../../capsule_net_dhcp/src/server/parse_req.rs"]
pub mod parse_req;
#[path = "../../capsule_net_dhcp/src/server/respond.rs"]
pub mod respond;
