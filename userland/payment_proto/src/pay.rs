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

//! PAY v2 (plan/21): listingId[32] | releaseHash[32] | payee[20] | unit[32]
//! | qty u64 | unitPrice u128 | callId[32] | chainId u64 | lp listing
//! | lp release. The reply is one VoucherRecord.

use crate::cursor::Cursor;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PayRequest<'a> {
    pub listing_id: [u8; 32],
    pub release_hash: [u8; 32],
    pub payee: [u8; 20],
    pub unit: [u8; 32],
    pub qty: u64,
    pub unit_price: u128,
    pub call_id: [u8; 32],
    pub chain_id: u64,
    /// The market's own strings, what OP_GET_COMMERCE looks the release up by.
    pub listing: &'a [u8],
    pub release: &'a [u8],
}

impl<'a> PayRequest<'a> {
    /// The request, or None: short, long, or an empty listing string.
    pub fn parse(b: &'a [u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let p = PayRequest {
            listing_id: r.fixed()?,
            release_hash: r.fixed()?,
            payee: r.fixed()?,
            unit: r.fixed()?,
            qty: r.u64()?,
            unit_price: r.u128()?,
            call_id: r.fixed()?,
            chain_id: r.u64()?,
            listing: r.lp()?,
            release: r.lp()?,
        };
        if p.listing.is_empty() {
            return None;
        }
        r.end(p)
    }
}
