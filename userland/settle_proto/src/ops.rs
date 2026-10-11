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

/// The registry name the meter's caller rule and every client use.
pub const SERVICE: &[u8] = b"nox_settle";

pub const OP_HEALTHCHECK: u16 = 1;
/// A signed raw transaction to send. Body: chain_id u64 | the bytes, at most
/// MAX_RAW_TX. Reply: its hash[32].
pub const OP_BROADCAST: u16 = 2;
/// openNonce(payer), the nonce the next open will use. Informational: the
/// ChannelOpened event is authoritative.
pub const OP_READ_NONCE: u16 = 3;
/// deposited, settled, expiry and closed for one channel (ChannelState).
pub const OP_CHANNEL_STATE: u16 = 4;
/// An address's ETH balance, next nonce and fee suggestions (Gas).
pub const OP_GAS: u16 = 5;
/// The ChannelOpened event a transaction emitted (Opened). EAGAIN while it
/// is unmined, ENOENT if it emitted none.
pub const OP_OPENED: u16 = 6;

/// The largest raw transaction BROADCAST takes.
pub const MAX_RAW_TX: usize = 128 * 1024;

pub const EAGAIN: i32 = -11;
pub const ENOENT: i32 = -2;
pub const EINVAL: i32 = -22;
