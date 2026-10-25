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

//! Network `ping`: one-shot ICMP echo to a host through the net.ip service.

mod emit;
mod icmp;
mod job;
mod lookup;
mod parse;
mod poll;
mod prepare;
mod probe;
mod refused;
mod resolve;
mod run;
mod send;
mod wire;

pub use emit::emit_probe;
pub use job::PingJob;
pub use prepare::prepare;
pub use run::run;

use wire::{
    DEADLINE_MS, HDR_LEN, IP_MAGIC, IP_SERVICE, OP_POLL_PACKET, OP_SEND_PACKET, PING_ID, PROTO_ICMP,
};
