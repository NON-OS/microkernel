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

//! The SOCKS front net.anon serves the browser, from the real sources. The
//! files that reach the manager are left out; the conversation is driven
//! against a tunnel the tests play.

#[path = "../../../capsule_net_anon/src/server/socks/conv.rs"]
pub mod conv;
#[path = "../../../capsule_net_anon/src/server/socks/frame.rs"]
pub mod frame;
#[path = "../../../capsule_net_anon/src/server/socks/front.rs"]
pub mod front;
#[path = "../../../capsule_net_anon/src/server/socks/kept.rs"]
mod kept;
#[path = "../../../capsule_net_anon/src/server/socks/relay.rs"]
mod relay;
#[path = "../../../capsule_net_anon/src/server/socks/rep.rs"]
mod rep;
#[path = "../../../capsule_net_anon/src/server/socks/reply.rs"]
mod reply;
#[path = "../../../capsule_net_anon/src/server/socks/request.rs"]
mod request;
#[path = "../../../capsule_net_anon/src/server/socks/stage.rs"]
pub mod stage;
#[path = "../../../capsule_net_anon/src/server/socks/stages.rs"]
mod stages;
#[path = "../../../capsule_net_anon/src/server/socks/tunnel.rs"]
pub mod tunnel;
#[path = "../../../capsule_net_anon/src/server/socks/turn.rs"]
mod turn;
#[path = "../../../capsule_net_anon/src/server/socks/wire.rs"]
mod wire;
