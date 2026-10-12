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

//! NoxListingRegistry's two events (plan/24), read strictly or not at all.
//! Published: listingId, releaseId and bondId are its indexed topics;
//! releaseHash, then the uri as an ABI string (the offset word 64, the
//! length, the bytes with zero padding, nothing more), 1 to MAX_URL bytes.
//! Withdrawn: listingId and releaseId, no data. Events are hints only.

use alloc::vec::Vec;

use crate::bytes::fixed;
use crate::fetch::MAX_URL;
pub use crate::registry_topics::{PUBLISHED, PUBLISHED_TOPIC, WITHDRAWN, WITHDRAWN_TOPIC};

/// One Published log, with the block it landed in.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Listed {
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub release_hash: [u8; 32],
    pub bond_id: [u8; 32],
    pub block: u64,
    pub uri: Vec<u8>,
}

fn small(w: &[u8]) -> Option<usize> {
    let high = w.get(..24)?.iter().all(|b| *b == 0);
    high.then_some(u64::from_be_bytes(fixed(w, 24)?))?.try_into().ok()
}

impl Listed {
    pub fn from_log(topics: &[[u8; 32]], data: &[u8], block: u64) -> Option<Listed> {
        let [t0, listing_id, release_id, bond_id] = topics else { return None };
        if *t0 != PUBLISHED_TOPIC || small(data.get(32..64)?)? != 64 {
            return None;
        }
        let len = small(data.get(64..96)?)?;
        let whole = len.div_ceil(32).checked_mul(32).and_then(|w| w.checked_add(96));
        if len == 0 || len > MAX_URL || Some(data.len()) != whole {
            return None;
        }
        let (uri, pad) = data.get(96..)?.split_at_checked(len)?;
        let release_hash = fixed(data, 0)?;
        pad.iter().all(|b| *b == 0).then(|| Listed {
            listing_id: *listing_id,
            release_id: *release_id,
            release_hash,
            bond_id: *bond_id,
            block,
            uri: uri.to_vec(),
        })
    }
}

/// A Withdrawn log's (listingId, releaseId).
pub fn withdrawn(topics: &[[u8; 32]], data: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    let [t0, listing_id, release_id] = topics else { return None };
    (*t0 == WITHDRAWN_TOPIC && data.is_empty()).then_some((*listing_id, *release_id))
}
