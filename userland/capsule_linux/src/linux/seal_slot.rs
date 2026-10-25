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

//! What a family notes to slot 0 of the kernel's seal. A family never noted
//! is dropped at its end (plan/10-seal.md), so every proved app is noted
//! (I-C6). Outside a box that is the app_id. The capsule's own BusyBox has an
//! app_id no verifier takes, all zero, and outside a box it is not noted.
//!
//! In a box the note is the app and the box together, so a receipt names
//! the box the run was held in (letter agent-02, gaps 1 and 2):
//! BLAKE3(BOXED_DOMAIN | app_id | box_hash). That is never zero, so a
//! BusyBox run in a box is sealed too.

pub const BOXED_DOMAIN: &[u8] = b"NONOS.agentproof.app.v1";

/// The note for `app_id`, run in the box `box_hash` names, if any; None for
/// a family that is not noted.
pub fn slot0(app_id: &[u8; 32], box_hash: Option<&[u8; 32]>) -> Option<[u8; 32]> {
    match box_hash {
        Some(h) => {
            let mut b = blake3::Hasher::new();
            b.update(BOXED_DOMAIN).update(app_id).update(h);
            Some(*b.finalize().as_bytes())
        }
        None => (*app_id != [0; 32]).then_some(*app_id),
    }
}
