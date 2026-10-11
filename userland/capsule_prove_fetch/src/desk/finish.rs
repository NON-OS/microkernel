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

//! The end of an upload or download. A download is kept only when its bytes
//! are a container whose anchor, under the listing asked for, is the anchor
//! asked for; else it is ANCHOR, and nothing of it is kept.

use alloc::vec::Vec;

use nonos_proof_abi::anchor_of;
use nonos_proof_abi::wire::fetch::{Stage, ANCHOR};

use super::transfer::Desk;

impl Desk {
    /// How the upload or download under way ended.
    pub fn finish(&mut self, got: Result<Vec<u8>, i32>) {
        let Some(t) = self.t.as_mut() else { return };
        let up = t.stage == Stage::Uploading;
        let keep = match &got {
            Ok(b) => up || anchor_of(&t.listing_id, b) == Some(t.anchor),
            Err(_) => false,
        };
        match got {
            Ok(b) if keep => {
                if !up {
                    t.bytes = b;
                }
                (t.stage, t.status, t.total) = (Stage::Held, 0, t.bytes.len() as u32);
            }
            Ok(_) => (t.stage, t.status, t.bytes) = (Stage::Failed, ANCHOR, Vec::new()),
            Err(s) => (t.stage, t.status, t.bytes) = (Stage::Failed, s, Vec::new()),
        }
    }
}
