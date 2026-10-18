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

//! An element by id, whole (id and length included).

use super::*;

/// An element by id, whole (id and length included). A walk that meets a
/// length running past the area stops there.
pub fn find_element(tags: &[u8], id: u8) -> Option<&[u8]> {
    let mut off = 0usize;
    while off + 2 <= tags.len() {
        let end = (off + 2).checked_add(tags[off + 1] as usize)?;
        if end > tags.len() {
            return None;
        }
        if tags[off] == id {
            return Some(&tags[off..end]);
        }
        off = end;
    }
    None
}

/// The rates to offer an access point, and what its selectors require.
#[derive(Clone, Copy)]
pub struct Rates {
    pub rates: [u8; 16],
    pub len: usize,
    /// The AP admits only 802.11n (HT) stations.
    pub requires_ht: bool,
    /// The AP admits only SAE hash-to-element.
    pub requires_h2e: bool,
}

impl Rates {
    /// The Supported Rates part (at most eight).
    pub fn supported(&self) -> &[u8] {
        &self.rates[..self.len.min(SUPPORTED_RATES_MAX)]
    }

    /// The Extended Supported Rates part (possibly empty).
    pub fn extended(&self) -> &[u8] {
        &self.rates[self.len.min(SUPPORTED_RATES_MAX)..self.len]
    }
}
