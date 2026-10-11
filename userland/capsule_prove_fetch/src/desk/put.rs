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

//! A PUT chunk into the desk. The first chunk opens a transfer, or reopens
//! its own caller's; the rest come in order, from that caller, for that
//! listing and total. The last makes the anchor, or NOT_CONTAINER.

use alloc::vec::Vec;

use nonos_proof_abi::anchor_of;
use nonos_proof_abi::wire::fetch::{Put, Stage, NOT_CONTAINER};

use super::errno::{EBUSY, EINVAL};
use super::transfer::{Desk, Transfer};

impl Desk {
    /// The anchor once the last chunk is in and is a container; the upload
    /// is then due. Chunks come in order, from one caller, for one listing.
    pub fn put(&mut self, from: u32, p: &Put) -> Result<Option<[u8; 32]>, i32> {
        if p.offset == 0 {
            let again =
                self.t.as_ref().is_some_and(|t| t.stage == Stage::Receiving && t.owner == from);
            if self.busy() && !again {
                return Err(EBUSY);
            }
            self.t = Some(Transfer {
                owner: from,
                listing_id: *p.listing_id,
                anchor: [0; 32],
                stage: Stage::Receiving,
                status: 0,
                total: p.total,
                bytes: Vec::with_capacity(p.total as usize),
            });
        }
        let Some(t) = self.t.as_mut().filter(|t| t.stage == Stage::Receiving) else {
            return Err(if self.busy() { EBUSY } else { EINVAL });
        };
        let ours = t.owner == from && t.listing_id == *p.listing_id && t.total == p.total;
        if !ours || t.bytes.len() != p.offset as usize {
            return Err(if t.owner == from { EINVAL } else { EBUSY });
        }
        t.bytes.extend_from_slice(p.chunk);
        if !p.last() {
            return Ok(None);
        }
        let Some(a) = anchor_of(&t.listing_id, &t.bytes) else {
            (t.stage, t.status, t.bytes) = (Stage::Failed, NOT_CONTAINER, Vec::new());
            return Err(NOT_CONTAINER);
        };
        (t.anchor, t.stage) = (a, Stage::Uploading);
        Ok(Some(a))
    }
}
