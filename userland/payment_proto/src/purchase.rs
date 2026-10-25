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

//! PURCHASE (op 19, plan/21, plan/27): the store-to-wallet mailbox, names
//! only, so a store that lies can name a listing and nothing else.
//!   POST 0 | chain_id u64 | lp(u16) listing | lp(u16) release (app.store)
//!   TAKE 1 -> chain_id | lp listing | lp release (the wallet)
//!   READY 2 | the names | orderId[32] (the wallet); TAKE_READY 3 (app.store)
//! The listing is never empty; an empty release is the listing's default.

use alloc::vec::Vec;

use crate::cursor::Cursor;
use crate::cursor_put::put_lp;

mod form;
pub use form::{parse_purchase, Purchase, Ready};

pub const PURCHASE_POST: u8 = 0;
pub const PURCHASE_TAKE: u8 = 1;
pub const PURCHASE_READY: u8 = 2;
pub const PURCHASE_TAKE_READY: u8 = 3;
/// The longest listing or release name the mailbox carries.
pub const MAX_NAME: usize = 128;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Names {
    pub chain_id: u64,
    pub listing: Vec<u8>,
    pub release: Vec<u8>,
}

impl Names {
    /// The names as TAKE answers them, and as POST carries them after its
    /// first byte.
    pub fn encode(&self) -> Option<Vec<u8>> {
        self.fits().then_some(())?;
        let mut out = Vec::from(self.chain_id.to_le_bytes());
        put_lp(&mut out, &self.listing)?;
        put_lp(&mut out, &self.release)?;
        Some(out)
    }

    pub fn post(&self) -> Option<Vec<u8>> {
        Some([&[PURCHASE_POST][..], &self.encode()?].concat())
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let chain_id = r.u64()?;
        let (listing, release) = (r.lp()?.to_vec(), r.lp()?.to_vec());
        let n = r.end(Names { chain_id, listing, release })?;
        n.fits().then_some(n)
    }

    pub(crate) fn fits(&self) -> bool {
        !self.listing.is_empty() && self.listing.len() <= MAX_NAME && self.release.len() <= MAX_NAME
    }
}
