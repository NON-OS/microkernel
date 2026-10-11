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

//! Whether a release's publisher signed the same commerce block the operator
//! listed. Only then are price and payee the publisher's word as well.

use crate::codec::commerce_hash_input;
use crate::ids::Blake3;
use crate::types::{CapsuleRelease, MarketplaceEntry};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum CommerceVerdict {
    Matches = 0,
    /// A schema 2 entry or release, which carries no commerce hash.
    NotV3 = 1,
    /// The release signed a different commerce block from the one listed.
    Mismatch = 2,
}

pub fn commerce_verdict(
    b: &impl Blake3,
    entry: &MarketplaceEntry,
    release: &CapsuleRelease,
) -> CommerceVerdict {
    let (Some(c), Some(x)) = (entry.commerce.as_ref(), release.ext.as_ref()) else {
        return CommerceVerdict::NotV3;
    };
    if b.blake3(&commerce_hash_input(c)) == x.commerce_hash {
        CommerceVerdict::Matches
    } else {
        CommerceVerdict::Mismatch
    }
}
