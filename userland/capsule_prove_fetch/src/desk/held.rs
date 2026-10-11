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

//! GET and STATUS against what fetch holds. A GET for a container not held
//! starts its download; one held is answered from memory, 64 KiB at a time.

use alloc::vec::Vec;

use nonos_proof_abi::wire::fetch::{status_body, Stage, PENDING};
use nonos_proof_abi::wire::GET_MAX;

use super::errno::{EBUSY, EINVAL, ENOENT};
use super::transfer::{Desk, Transfer};

impl Desk {
    fn this(&mut self, listing_id: &[u8; 32], anchor: &[u8; 32]) -> Option<&mut Transfer> {
        let t = self.t.as_mut()?;
        (t.stage != Stage::Receiving && t.anchor == *anchor && t.listing_id == *listing_id)
            .then_some(t)
    }

    /// The container from `offset` once held; PENDING while it comes, a
    /// download started for one not held, which a failed one is again.
    pub fn get(
        &mut self,
        listing_id: &[u8; 32],
        anchor: &[u8; 32],
        at: u32,
    ) -> Result<Vec<u8>, i32> {
        if let Some(t) = self.this(listing_id, anchor) {
            match t.stage {
                Stage::Held => {
                    let rest = t.bytes.get(at as usize..).ok_or(EINVAL)?;
                    return Ok(rest[..rest.len().min(GET_MAX)].to_vec());
                }
                Stage::Failed => {}
                _ => return Err(PENDING),
            }
        } else if self.busy() {
            return Err(EBUSY);
        }
        let (stage, status, total, bytes) = (Stage::Downloading, 0, 0, Vec::new());
        let (listing_id, anchor) = (*listing_id, *anchor);
        self.t = Some(Transfer { owner: 0, listing_id, anchor, stage, status, total, bytes });
        Err(PENDING)
    }

    /// The reply's status and body for STATUS: ENOENT for an anchor not here.
    pub fn status(&self, anchor: &[u8; 32]) -> (i32, [u8; 5]) {
        match self.t.as_ref().filter(|t| t.stage != Stage::Receiving && t.anchor == *anchor) {
            Some(t) => (t.status, status_body(t.stage, t.total)),
            None => (ENOENT, [0; 5]),
        }
    }
}
