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

//! One signed voucher, PAY's reply and DRAIN's row, VOUCHER_RECORD_LEN bytes:
//!
//!   channel_id[32] | payer[20] | payee[20] | listing_id[32] | unit[32]
//!   | cumulative u128 | seq u64 | expiry u64 | usage_root[32] | sig[65]

use alloc::vec::Vec;

pub const VOUCHER_RECORD_LEN: usize = 32 + 20 + 20 + 32 + 32 + 16 + 8 + 8 + 32 + 65;
/// Signed vouchers in one DRAIN page.
pub const DRAIN_ROWS_MAX: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VoucherRecord {
    pub channel_id: [u8; 32],
    pub payer: [u8; 20],
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    pub unit: [u8; 32],
    pub cumulative: u128,
    pub seq: u64,
    pub expiry: u64,
    pub usage_root: [u8; 32],
    pub sig: [u8; 65],
}

impl VoucherRecord {
    pub fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.channel_id);
        out.extend_from_slice(&self.payer);
        out.extend_from_slice(&self.payee);
        out.extend_from_slice(&self.listing_id);
        out.extend_from_slice(&self.unit);
        out.extend_from_slice(&self.cumulative.to_le_bytes());
        out.extend_from_slice(&self.seq.to_le_bytes());
        out.extend_from_slice(&self.expiry.to_le_bytes());
        out.extend_from_slice(&self.usage_root);
        out.extend_from_slice(&self.sig);
    }
}
