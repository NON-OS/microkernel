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

//! The NNET frame service every NIC driver offers net.core and net.l2.

mod answer;
mod encode;
mod frames;
mod stats;
mod wire;

pub use answer::answer;
pub use encode::{reply, status};
pub use stats::Stats;
pub use wire::OP_TX_PACKET;
pub use wire::{decode, Request, DATA_AT, HDR_LEN, RX_PREFIX_LEN, STATS_LEN, STATUS_LEN};
pub use wire::{E_AGAIN, E_INVAL, E_IO, E_MSGSIZE, MAGIC, VERSION};
pub use wire::{OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_STATS};
