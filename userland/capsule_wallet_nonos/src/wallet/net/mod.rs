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

mod bounds;
mod call;
mod constants;
mod health;
pub mod last_read;
mod link;
mod lookup;
mod probe_rpc_tcp;
mod probe_status;
mod probe_tls_rpc;
pub mod read_snapshot;
mod read_tls_flight;
mod resolve_eth;
mod route_text;
mod rtc_stamp;
pub mod send_cap;
mod socket_close;
mod socket_connect;
mod socket_open;
mod socket_recv;
mod socket_send;
mod status;
pub mod step;

/* The route type, named once here so the pure files below reach it as
 * `super::Route`, the way wallet_proofs compiles them. */
use nonos_route_link::Route;

pub use route_text::{route_part, route_value};
pub use status::NetStatus;
