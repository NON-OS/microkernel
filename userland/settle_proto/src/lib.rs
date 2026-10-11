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

//! capsule_nox_settle's wire (plan/26, plan/42): market_proto's 20-byte
//! header under the magic "NSTL", a reply's payload starting with a status,
//! every body little-endian with every length explicit. Settle holds no keys:
//! it reads the chain and broadcasts what others signed.
//!
//! This holds what the meter needs first (CHANNEL_STATE, GAS, BROADCAST,
//! OPENED). READ, INTENT and FETCH land with settle itself (B4).

#![no_std]

extern crate alloc;

mod chain;
mod channel_state;
mod frame;
mod gas;
mod header;
mod opened;
mod ops;

pub use chain::{chain_hash, parse_chain_hash, ChainHash};
pub use channel_state::{ChannelState, CHANNEL_STATE_LEN};
pub use frame::{reply, reply_body, request, ReplyError};
pub use gas::{Gas, GAS_LEN};
pub use header::{parse_header, Header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use opened::{Opened, OPENED_LEN};
pub use ops::*;
