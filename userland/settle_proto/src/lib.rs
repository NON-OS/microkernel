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

//! capsule_nox_settle's wire. Requests are seq u32 | op u16 | reserved u16 |
//! body; replies are seq u32 | status i32 | body; all little-endian, as the
//! meter's own. Settle holds no keys: it reads the chain and broadcasts what
//! others signed.

#![no_std]

mod channel_state;
mod ops;

pub use channel_state::{channel_state_reply, channel_state_request, parse_channel_state};
pub use channel_state::{ChannelState, CHANNEL_STATE_LEN};
pub use ops::*;
