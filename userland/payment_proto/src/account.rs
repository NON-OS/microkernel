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

//! ACCOUNT, for the wallet. The request is one byte, the first channel
//! wanted. The reply pages the channel table ACCOUNT_ROWS_MAX at a time:
//!
//!   flags u8 | address[20] | cap u128 | spent u128 | expiry u64 | left u128
//!   | total u8 | rows u8 | rows x (channel_id[32] | listing_id[32]
//!   | chain_id u64 | deposit u128
//!   | settled u128 | cumulative u128 | expiry u64 | frozen u8)
//!
//! READY when the meter's key is open (until then address is zero); KEPT when
//! this boot persists. frozen: settle reported settled above what this meter
//! signed, from rolled-back state or a lying RPC. listing_id (v1.4.48) lets the
//! wallet say a listing already has an open channel.

use alloc::vec::Vec;

pub const ACCOUNT_ROWS_MAX: usize = 32;
pub const ACCOUNT_ROW_LEN: usize = 32 + 32 + 8 + 16 * 3 + 8 + 1;
pub const READY: u8 = 1 << 0;
pub const KEPT: u8 = 1 << 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AccountRow {
    pub channel_id: [u8; 32],
    pub listing_id: [u8; 32],
    pub chain_id: u64,
    pub deposit: u128,
    pub settled: u128,
    pub cumulative: u128,
    pub expiry: u64,
    pub frozen: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Account {
    pub flags: u8,
    pub address: [u8; 20],
    pub cap: u128,
    pub spent: u128,
    pub expiry: u64,
    pub left: u128,
    /// Channels in the whole table; `rows` is one page of them.
    pub total: u8,
    pub rows: Vec<AccountRow>,
}
