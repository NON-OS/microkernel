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

//! The second bulk pair a slot may hold beside the first: UAS's command
//! and status pipes on one pair, its data pipes on the other (0x23). Bulk
//! ops name the pair in their second byte, which older clients send as 0.
//! Pure, for xhci_proofs.

use super::resources::SlotResources;
use crate::controller::BulkPipes;

impl SlotResources {
    /// Whether a second pair at `dcis` fits: one more a slot, beside the
    /// first, on endpoints the first does not hold.
    pub fn takes_second_bulk(&self, dcis: (u8, u8)) -> bool {
        let clear =
            |p: &BulkPipes| ![p.dci_in, p.dci_out].iter().any(|d| [dcis.0, dcis.1].contains(d));
        self.bulk2.is_none() && self.bulk.as_ref().is_none_or(clear)
    }

    /// The first pair or, `second`, the second.
    pub fn pair_mut(&mut self, second: bool) -> Option<&mut BulkPipes> {
        if second {
            self.bulk2.as_mut()
        } else {
            self.bulk.as_mut()
        }
    }

    /// A class driver holds a bulk pair on it.
    pub fn has_bulk(&self) -> bool {
        self.bulk.is_some() || self.bulk2.is_some()
    }
}

/// A bulk op's pair byte: 0 the first pair, 1 the second; anything else
/// is no pair. Older clients send 0 there, which keeps them on the first.
pub fn pair_of(byte: u8) -> Option<bool> {
    match byte {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}
