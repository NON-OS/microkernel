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
// 3 was READ_NONCE; READ's view 14 (openNonce) replaces it, and 3 is EINVAL.
/// deposited, settled, expiry and closed for one channel (ChannelState).
pub const OP_CHANNEL_STATE: u16 = 4;
/// An address's ETH balance, next nonce and fee suggestions (Gas).
pub const OP_GAS: u16 = 5;
/// The ChannelOpened event a transaction emitted (Opened). EAGAIN while it
/// is unmined, ENOENT if it emitted none.
pub const OP_OPENED: u16 = 6;

/// One config contract's view (Read): the raw ABI return.
pub const OP_READ: u16 = 7;
/// Check and quote a transaction its key holder built (Intent, Quote).
pub const OP_INTENT: u16 = 8;
/// A release's signing bytes, verified, in pages (Fetch, Page).
pub const OP_FETCH: u16 = 9;
/// One drain round now: the meter's vouchers to the chain (Settled).
pub const OP_SETTLE: u16 = 10;
/// A listing's current releases from the registry's logs, as hints (Listing).
pub const OP_LISTING: u16 = 11;
/// An order's key from the escrow's Delivered log, for payment (v1.4.46).
pub const OP_DELIVERED: u16 = 12;

/// The largest raw transaction BROADCAST takes.
pub const MAX_RAW_TX: usize = 128 * 1024;

pub const EAGAIN: i32 = -11;
pub const ENOENT: i32 = -2;
pub const EINVAL: i32 = -22;
/// A caller the op does not answer (callers.rs).
pub const EPERM: i32 = -1;
/// The RPC failed, or two hosts disagree.
pub const EIO: i32 = -5;
/// Bytes that are not what was asked for: a wrong hash or chain id.
pub const EPROTO: i32 = -71;
/// A release past MAX_RELEASE.
pub const EFBIG: i32 = -27;
/// The anonymous route is down; settle never falls back to Direct.
pub const ENETDOWN: i32 = -100;
