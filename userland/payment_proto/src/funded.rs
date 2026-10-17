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

//! FUNDED (op 21, the plan): app.store asks, before it posts a voucher
//! listing to the wallet, whether the listing's channel already takes one
//! more install; the answer is one bit and nothing else.
//!
//!   chain_id u64 | lp(u16) listing   -> FUNDED_NO 0 or FUNDED_YES 1
//!
//! The listing is the market's own string, what OP_GET_COMMERCE looks the
//! listing up by, never empty. No amount: the meter prices the install
//! from the listing's commerce block, so the bit cannot bisect a deposit.

use alloc::vec::Vec;

use crate::cursor::Cursor;
use crate::cursor_put::{flag, put_lp};
use crate::purchase::MAX_NAME;

pub const FUNDED_NO: u8 = 0;
pub const FUNDED_YES: u8 = 1;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FundedAsk {
    pub chain_id: u64,
    pub listing: Vec<u8>,
}

impl FundedAsk {
    pub fn encode(&self) -> Option<Vec<u8>> {
        self.fits().then_some(())?;
        let mut out = Vec::from(self.chain_id.to_le_bytes());
        put_lp(&mut out, &self.listing)?;
        Some(out)
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (chain_id, listing) = (r.u64()?, r.lp()?.to_vec());
        let ask = r.end(FundedAsk { chain_id, listing })?;
        ask.fits().then_some(ask)
    }

    fn fits(&self) -> bool {
        !self.listing.is_empty() && self.listing.len() <= MAX_NAME
    }
}

/// The reply's body: exactly one byte, 0 or 1.
pub fn parse_funded(b: &[u8]) -> Option<bool> {
    match b {
        [one] => flag(*one),
        _ => None,
    }
}
