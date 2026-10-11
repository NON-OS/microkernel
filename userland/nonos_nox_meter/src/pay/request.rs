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

//! PAY v2's body (plan/21, v1.2), little-endian:
//!
//!   listingId[32] | releaseHash[32] | payee[20] | unit[32] | qty u64
//!   | unitPrice u128 | callId[32] | chainId u64 | lp listing | lp release
//!
//! lp is a u32 length and the bytes, as the market's own bodies are. The two
//! strings are what OP_GET_COMMERCE looks the release up by.

use crate::error::MeterError;

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
    pub listing: &'a [u8],
    pub release: &'a [u8],
}

const FIXED: usize = 32 + 32 + 20 + 32 + 8 + 16 + 32 + 8;

fn at<const N: usize>(b: &[u8], from: usize) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(&b[from..from + N]);
    out
}

fn lp(b: &[u8]) -> Option<(&[u8], &[u8])> {
    let len = u32::from_le_bytes(b.get(..4)?.try_into().ok()?) as usize;
    let end = 4usize.checked_add(len)?;
    Some((b.get(4..end)?, &b[end..]))
}

/// The request, or Corrupt: short, long, or a length past the end.
pub fn parse_pay(b: &[u8]) -> Result<PayRequest<'_>, MeterError> {
    let fixed = b.get(..FIXED).ok_or(MeterError::Corrupt)?;
    let (listing, rest) = lp(&b[FIXED..]).ok_or(MeterError::Corrupt)?;
    let (release, rest) = lp(rest).ok_or(MeterError::Corrupt)?;
    if !rest.is_empty() || listing.is_empty() {
        return Err(MeterError::Corrupt);
    }
    Ok(PayRequest {
        listing_id: at(fixed, 0),
        release_hash: at(fixed, 32),
        payee: at(fixed, 64),
        unit: at(fixed, 84),
        qty: u64::from_le_bytes(at(fixed, 116)),
        unit_price: u128::from_le_bytes(at(fixed, 124)),
        call_id: at(fixed, 140),
        chain_id: u64::from_le_bytes(at(fixed, 172)),
        listing,
        release,
    })
}
