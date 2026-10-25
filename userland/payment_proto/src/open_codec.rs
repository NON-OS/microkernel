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

use alloc::vec::Vec;

use crate::cursor::Cursor;
use crate::open::{OpenConfirm, OpenStart, OPEN_CONFIRM, OPEN_START};

impl OpenStart {
    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        out.extend_from_slice(&self.payee);
        out.extend_from_slice(&self.listing_id);
        out.extend_from_slice(&self.unit);
        out.extend_from_slice(&self.amount.to_le_bytes());
        out.extend_from_slice(&self.expiry.to_le_bytes());
        out.extend_from_slice(&self.reclaim_to);
    }

    fn read(r: &mut Cursor) -> Option<Self> {
        let (chain_id, payee, listing_id, unit) = (r.u64()?, r.fixed()?, r.fixed()?, r.fixed()?);
        let (amount, expiry, reclaim_to) = (r.u128()?, r.u64()?, r.fixed()?);
        Some(OpenStart { chain_id, payee, listing_id, unit, amount, expiry, reclaim_to })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = alloc::vec![OPEN_START];
        self.put(&mut out);
        out
    }
}

impl OpenConfirm {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = alloc::vec![OPEN_CONFIRM];
        self.start.put(&mut out);
        out.extend_from_slice(&self.open_tx);
        out
    }
}

/// OPEN's body: Ok(start) or Err(confirm), or None.
pub fn parse_open(b: &[u8]) -> Option<Result<OpenStart, OpenConfirm>> {
    let mut r = Cursor::new(b);
    let form = r.u8()?;
    let start = OpenStart::read(&mut r)?;
    match form {
        OPEN_START => r.end(Ok(start)),
        OPEN_CONFIRM => {
            let open_tx = r.fixed()?;
            r.end(Err(OpenConfirm { start, open_tx }))
        }
        _ => None,
    }
}
