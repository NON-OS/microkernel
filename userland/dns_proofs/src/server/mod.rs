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

//! The capsule's server module, each file its own, made reachable from the
//! proofs. runner.rs, the inbox loop that never returns, is left out.

#[path = "../../../capsule_net_dns/src/server/authz.rs"]
pub mod authz;
#[path = "../../../capsule_net_dns/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../../capsule_net_dns/src/server/parse_req.rs"]
pub mod parse_req;
#[path = "../../../capsule_net_dns/src/server/respond.rs"]
pub mod respond;
