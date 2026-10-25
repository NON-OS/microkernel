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

//! A record changed by someone without the host keys.

use super::record::Rec;

/// The good record's footer kept as it was over `entries`: what someone
/// without the host keys can make of a record. With `rechain` the chain
/// value is recomputed for the new entries, which they can also do.
pub fn forge(good: &Rec, entries: Vec<Vec<u8>>, rechain: bool) -> Vec<u8> {
    let signed = good.bytes();
    let sig = &signed[signed.len() - 3373..];
    let changed = Rec { entries, header: good.header.clone(), ..*good };
    let mut out = changed.unsigned();
    if !rechain {
        let n = out.len();
        out[n - 64..].copy_from_slice(&good.chain_after(good.entries.len() - 1));
    }
    [out, sig.to_vec()].concat()
}

impl Rec {
    /// The header and entries alone, as a crash leaves them.
    pub fn no_footer(&self) -> Vec<u8> {
        [self.header.clone(), self.entries.concat()].concat()
    }
}
