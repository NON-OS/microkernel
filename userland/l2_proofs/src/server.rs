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

//! net.l2's `server` module as far as a refused frame goes: the reply the loop
//! answers it with. The header decode is in the protocol module; the handlers
//! and the receive loop stay out.

// The capsule's own layout: respond/mod.rs holds respond/respond.rs.
#[allow(clippy::module_inception)]
#[path = "../../capsule_net_l2/src/server/respond/mod.rs"]
pub mod respond;
