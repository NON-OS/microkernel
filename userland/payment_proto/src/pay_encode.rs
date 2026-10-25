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

use crate::cursor_put::put_lp;
use crate::pay::PayRequest;

impl PayRequest<'_> {
    /// The body, or None if a string is longer than a u16 can say.
    pub fn encode(&self) -> Option<Vec<u8>> {
        let urls = self.listing.len().saturating_add(self.release.len());
        let mut out = Vec::with_capacity(urls.saturating_add(180));
        out.extend_from_slice(&self.listing_id);
        out.extend_from_slice(&self.release_hash);
        out.extend_from_slice(&self.payee);
        out.extend_from_slice(&self.unit);
        out.extend_from_slice(&self.qty.to_le_bytes());
        out.extend_from_slice(&self.unit_price.to_le_bytes());
        out.extend_from_slice(&self.call_id);
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        put_lp(&mut out, self.listing)?;
        put_lp(&mut out, self.release)?;
        Some(out)
    }
}
