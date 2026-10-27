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

//! What the bridge holds of one guest's selections. Each guest has its own,
//! in its own scene, so no guest can name another's source or offer. Pure,
//! so the host proofs hold it.

use alloc::vec::Vec;

/// The source an offer stands for when it is the desktop's clipboard.
pub const DESKTOP: u32 = 0;

#[derive(Default)]
pub struct Clip {
    /// Each of the guest's live sources, with the mime types it offered.
    pub sources: Vec<(u32, Vec<Vec<u8>>)>,
    /// The source of the clipboard selection, then of the primary one.
    pub held: [Option<u32>; 2],
    /// Each offer the client holds, with the source it stands for.
    pub offers: Vec<(u32, u32)>,
    /// When the window was last given a key or a button, in the family's
    /// milliseconds. The router sends input only while it has focus.
    pub input_ms: u64,
    /// The bridge's read of a text selection for the desktop (desk.rs).
    pub reading: Option<Reading>,
    /// A digest of the text last copied to the desktop (desk_copy.rs).
    pub copied: Option<u64>,
}

/// A pipe the bridge holds the read end of, and what came through it.
pub struct Reading {
    pub slot: usize,
    pub fd: u64,
    pub bytes: Vec<u8>,
}

impl Clip {
    /// The offer `id`'s source, while the client holds it.
    pub fn source_of(&self, offer: u32) -> Option<u32> {
        self.offers.iter().find(|o| o.0 == offer).map(|o| o.1)
    }
}
