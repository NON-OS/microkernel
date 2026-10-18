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

//! The chip's key store and which peer holds each pairwise CAM slot.

use crate::regs::Mmio;

/// `RTW_SEC_DEFAULT_KEY_NUM`: CAM entries 0..3 are reserved for group keys keyed
/// by their 802.11 key index; pairwise keys start here.
pub(super) const PAIRWISE_BASE: u8 = 4;
/// How many pairwise peers this station tracks. A client associates with one AP;
/// a small margin covers a roam that installs a new key before removing the old.
pub(super) const PAIRWISE_SLOTS: usize = 4;

/// The chip's key store: which peer occupies each pairwise CAM slot, over the
/// register window `M`.
pub struct RtlKeys<M: Mmio> {
    pub(super) mmio: M,
    pub(super) pairwise: [Option<[u8; 6]>; PAIRWISE_SLOTS],
}

impl<M: Mmio> RtlKeys<M> {
    pub const fn new(mmio: M) -> Self {
        Self { mmio, pairwise: [None; PAIRWISE_SLOTS] }
    }

    // The slot a peer already holds, or the first free one.
    pub(super) fn slot_for(&self, peer: &[u8; 6]) -> Option<usize> {
        if let Some(i) = self.pairwise.iter().position(|p| p.as_ref() == Some(peer)) {
            return Some(i);
        }
        self.pairwise.iter().position(Option::is_none)
    }
}
