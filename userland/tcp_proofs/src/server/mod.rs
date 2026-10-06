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
//! proofs. Only runner.rs is left out: it is the inbox loop, which never
//! returns, and on the host the proofs are its caller.

#[path = "../../../capsule_net_tcp/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../../capsule_net_tcp/src/server/orphans.rs"]
pub mod orphans;
#[path = "../../../capsule_net_tcp/src/server/parse_req.rs"]
pub mod parse_req;
#[path = "../../../capsule_net_tcp/src/server/persist.rs"]
pub mod persist;
#[path = "../../../capsule_net_tcp/src/server/respond.rs"]
pub mod respond;
#[path = "../../../capsule_net_tcp/src/server/retransmit.rs"]
pub mod retransmit;
#[path = "../../../capsule_net_tcp/src/server/room.rs"]
pub mod room;
#[path = "../../../capsule_net_tcp/src/server/sender.rs"]
pub mod sender;
#[path = "../../../capsule_net_tcp/src/server/tcp_rx/mod.rs"]
pub mod tcp_rx;
#[path = "../../../capsule_net_tcp/src/server/tcp_tx.rs"]
pub mod tcp_tx;
#[path = "../../../capsule_net_tcp/src/server/tick.rs"]
pub mod tick;
