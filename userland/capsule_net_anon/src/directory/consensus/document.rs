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

//! A parsed consensus: its validity window, its weights and its relays.

extern crate alloc;

use alloc::vec::Vec;

use crate::path::Weights;

use super::entry::Entry;
use super::signature::Signature;

/// A microdescriptor flavour consensus.
pub struct Consensus {
    pub valid_after: u64,
    pub fresh_until: u64,
    pub valid_until: u64,
    pub weights: Weights,
    pub entries: Vec<Entry>,
    pub signatures: Vec<Signature>,
    /// The byte range the signatures are computed over, as offsets into the
    /// document this was parsed from.
    pub signed: (usize, usize),
}

impl Consensus {
    /// An empty document over a known signed range, for the scan to fill in.
    pub fn empty(signed: (usize, usize)) -> Self {
        Self {
            valid_after: 0,
            fresh_until: 0,
            valid_until: 0,
            weights: Weights::default(),
            entries: Vec::new(),
            signatures: Vec::new(),
            signed,
        }
    }

    /// Whether `now` is inside the document's validity window.
    ///
    pub fn valid_at(&self, now: u64) -> bool {
        now >= self.valid_after && now < self.valid_until
    }
}

/// Whether a newer consensus should be fetched.
///
pub fn is_stale(fresh_until: u64, now: u64) -> bool {
    now >= fresh_until
}
