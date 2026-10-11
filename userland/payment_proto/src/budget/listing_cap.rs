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

//! LISTING_CAP (op 20, plan/21, v1.4.43), from the wallet alone:
//! listing_id[32] | cap u128, 48 bytes; no reply body. One listing's cap,
//! the rest of the budget kept; a cap of 0 removes it; ENFILE past
//! MAX_LISTING_CAPS.

use alloc::vec::Vec;

use crate::cursor::Cursor;

pub const LISTING_CAP_LEN: usize = 32 + 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ListingCapAsk {
    pub listing_id: [u8; 32],
    pub cap: u128,
}

impl ListingCapAsk {
    pub fn encode(&self) -> Vec<u8> {
        [&self.listing_id[..], &self.cap.to_le_bytes()].concat()
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (listing_id, cap) = (r.fixed()?, r.u128()?);
        r.end(ListingCapAsk { listing_id, cap })
    }
}
