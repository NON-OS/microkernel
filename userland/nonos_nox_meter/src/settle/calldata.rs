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

//! The calldata the meter encodes itself for settle, topUp and close
//!: settle never supplies any. Selectors pinned by tests
//! against their signatures.

use alloc::vec::Vec;

use crate::voucher::Voucher;
use crate::words::{u128_word, u64_word};

/// settle((bytes32,address,address,bytes32,bytes32,uint256,uint64,uint64,bytes32)[],bytes[])
pub const SETTLE_SELECTOR: [u8; 4] = [0x13, 0x50, 0x8e, 0xc8];
/// topUp(bytes32,uint256)
pub const TOPUP_SELECTOR: [u8; 4] = [0xb6, 0x76, 0x44, 0xb9];
/// close(bytes32)
pub const CLOSE_SELECTOR: [u8; 4] = [0x39, 0xc7, 0x9e, 0x0c];

/// abi.encode of settle(vouchers, sigs): two head offsets; the voucher
/// array (each tuple static, nine words inline); then the sigs array, an
/// offset per entry, each 65 bytes as length and three padded words.
pub fn settle_data(signed: &[(Voucher, [u8; 65])]) -> Vec<u8> {
    let n = signed.len();
    let mut out = Vec::with_capacity(4 + 64 + 32 + n * 288 + 32 + n * 160);
    out.extend_from_slice(&SETTLE_SELECTOR);
    out.extend_from_slice(&u64_word(64));
    out.extend_from_slice(&u64_word(64 + 32 + 288 * n as u64));
    out.extend_from_slice(&u64_word(n as u64));
    signed.iter().flat_map(|(v, _)| v.words()).for_each(|w| out.extend_from_slice(&w));
    out.extend_from_slice(&u64_word(n as u64));
    for i in 0..n as u64 {
        out.extend_from_slice(&u64_word(32 * n as u64 + 128 * i));
    }
    for (_, sig) in signed {
        out.extend_from_slice(&u64_word(65));
        out.extend_from_slice(sig);
        out.extend_from_slice(&[0u8; 31]);
    }
    out
}

pub fn topup_data(channel_id: &[u8; 32], amount: u128) -> [u8; 68] {
    let mut out = [0u8; 68];
    out[..4].copy_from_slice(&TOPUP_SELECTOR);
    out[4..36].copy_from_slice(channel_id);
    out[36..].copy_from_slice(&u128_word(amount));
    out
}

pub fn close_data(channel_id: &[u8; 32]) -> [u8; 36] {
    let mut out = [0u8; 36];
    out[..4].copy_from_slice(&CLOSE_SELECTOR);
    out[4..].copy_from_slice(channel_id);
    out
}
