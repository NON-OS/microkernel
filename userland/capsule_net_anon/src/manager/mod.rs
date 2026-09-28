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

//! Everything net.anon holds between requests, and the ticks that advance it.

mod batch;
mod body_closed;
mod body_step;
mod circuit_tick;
mod dir_certs;
mod dir_consensus;
mod dir_join;
mod dir_load;
mod dir_micro;
mod dir_quorum;
mod dir_tick;
mod guard;
mod http;
mod http_exchange;
mod inbound;
mod link_lost;
mod link_stage;
mod link_tick;
mod out;
mod pump;
mod read_body;
mod relays;
mod retire;
mod roll;
mod state;

pub use circuit_tick::tick as circuit_tick;
pub use dir_tick::tick as directory_tick;
pub use link_tick::tick as link_tick;
pub use out::{open_stream, send_data, send_end, sendme_tick, SendError};
pub use pump::tick as pump_tick;
pub use retire::tick as retire_tick;
pub use state::{Bootstrap, Manager};
