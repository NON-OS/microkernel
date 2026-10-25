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
//! The meter's state file, little-endian and always STATE_LEN bytes, because
//! the vfs replaces a persisted record only with one of the same length:
//!
//!   "NXMT" | version u16 = 5 | reserved u16 = 0
//!   | last settle: chain_id u64 | nonce u64 (chain 0: none) | last_retry u64
//!   | budget: cap u128 | spent u128 | expiry u64
//!   | n u16 | MAX_LISTING_CAPS slots of (listing_id[32] | cap u128 | spent u128)
//!   | m u16 | MAX_CHANNELS slots of a channel (CHANNEL_LEN bytes)
//!
//! A channel is id[32] | payee[20] | listing_id[32] | chain_id u64
//! | open_nonce u64 | unit[32] | deposit u128 | settled u128 | cumulative u128
//! | seq u64 | expiry u64 | usage_root[32] | sig[65] | frozen u8 (0 or 1)
//! | carried u128 | reclaim_to[20].
//! The first n and m slots are used and every byte of the rest is zero. A file
//! that does not read back exactly is refused, and the meter charges nothing
//! rather than trust a guess. Version 2 added `frozen` (v1.3, E13); version 3
//! the last settle and `carried` (v1.4.7); version 4 `last_retry` (v1.4.12);
//! version 5 `reclaim_to` (v1.4.60).

use crate::budget::MAX_LISTING_CAPS;
use crate::channel::MAX_CHANNELS;

pub const MAGIC: [u8; 4] = *b"NXMT";
pub const VERSION: u16 = 5;
pub const CHANNEL_LEN: usize = 32 + 20 + 32 + 8 + 8 + 32 + 16 * 3 + 8 + 8 + 32 + 65 + 1 + 16 + 20;
pub const LISTING_LEN: usize = 32 + 16 + 16;
pub const HEADER_LEN: usize = 4 + 2 + 2 + 8 + 8 + 8 + 16 + 16 + 8;
pub const STATE_LEN: usize =
    HEADER_LEN + 2 + MAX_LISTING_CAPS * LISTING_LEN + 2 + MAX_CHANNELS * CHANNEL_LEN;
