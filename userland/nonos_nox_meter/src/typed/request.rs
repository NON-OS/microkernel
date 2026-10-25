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

//! The keyring's OP_SIGN_TYPED_DATA request, payload only: pid u32,
//! key u32, entry u16, reserved u16 = 0, chain_id u64, n u8, n words; all LE.
//! The caller supplies no type string and no domain: the keyring's own table
//! holds them for the entry, and refuses an entry whose contract is None.

use alloc::vec::Vec;

use crate::voucher::Voucher;

pub const OP_SIGN_TYPED_DATA: u16 = 30;
/// The Voucher row of the keyring's table; only the `payment` service may use
/// it, and only with a payer word that is the signing key's own address.
pub const VOUCHER_ENTRY: u16 = 1;

pub fn voucher_request(pid: u32, key: u32, chain_id: u64, v: &Voucher) -> Vec<u8> {
    let words = v.words();
    let mut out = Vec::with_capacity(21 + 32 * words.len());
    out.extend_from_slice(&pid.to_le_bytes());
    out.extend_from_slice(&key.to_le_bytes());
    out.extend_from_slice(&VOUCHER_ENTRY.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&chain_id.to_le_bytes());
    out.push(words.len() as u8);
    for w in &words {
        out.extend_from_slice(w);
    }
    out
}
